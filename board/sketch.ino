/*
  This is the code for the AirGradient DIY Air Quality Sensor with an ESP8266 Microcontroller.

  It is a high quality sensor showing PM2.5, CO2, Temperature and Humidity on a small display and can send data over Wifi.

  For build instructions please visit https://www.airgradient.com/diy/

  Compatible with the following sensors:
  Plantower PMS5003 (Fine Particle Sensor)
  SenseAir S8 (CO2 Sensor)
  SHT30/31 (Temperature/Humidity Sensor)

  Please install ESP8266 board manager (tested with version 3.0.0)

  The codes needs the following libraries installed:
  "WifiManager by tzapu, tablatronix" tested with Version 2.0.3-alpha
  "ESP8266 and ESP32 OLED driver for SSD1306 displays by ThingPulse, Fabrice Weinberg" tested with Version 4.1.0

  Configuration:
  Please set in the code below which sensor you are using and if you want to connect it to WiFi.

  If you are a school or university contact us for a free trial on the AirGradient platform.
  https://www.airgradient.com/schools/

  MIT License
*/

#include <AirGradient.h>
#include <WiFiManager.h>
#include <ESP8266WiFi.h>
#include <ESP8266HTTPClient.h>

#include <Wire.h>
#include "SSD1306Wire.h"

AirGradient ag = AirGradient();

SSD1306Wire display(0x3c, SDA, SCL);

// set sensors that you do not use to false
boolean hasPM = true;
boolean hasCO2 = true;
boolean hasSHT = true;

const char* ssid = "";
const char* password = "";

// change if you want to send the data to another server
String APIROOT = "";

IPAddress server(10, 0, 0, 152);

unsigned long startMillis;  //some global variables available anywhere in the program
unsigned long currentMillis;
const unsigned long period = 60000; //60 seconds (every minute)

void setup() {
  Serial.begin(9600);

  display.init();
  display.flipScreenVertically();
  showTextRectangle("Init", String(ESP.getChipId(), HEX), true);

  if (hasPM) ag.PMS_Init();
  if (hasCO2) ag.CO2_Init();
  if (hasSHT) ag.TMP_RH_Init(0x44);


  // Set WiFi mode to client (without this it may try to act as an AP).
  WiFi.mode(WIFI_STA);
  // Setup and wait for WiFi.
  WiFi.begin(ssid, password);

  while (WiFi.status() != WL_CONNECTED) {
    delay(500);
    showTextRectangle("Trying to", "connect...", true);
  }

  startMillis = millis();  //initial start time
}

void loop() {

  // create payload
  String payload = "{\"chip\":" + String(ESP.getChipId()) + ",";

  if (hasPM) {
    int PM2 = ag.getPM2_Raw();
    payload = payload + "\"pm02\":" + String(PM2);
    showTextRectangle("PM2", String(PM2), false);
    delay(3000);
  }

  if (hasCO2) {
    if (hasPM) payload = payload + ",";
    int CO2 = ag.getCO2_Raw();
    payload = payload + "\"rco2\":" + String(CO2);
    showTextRectangle("CO2", String(CO2), false);
    delay(3000);
  }

  if (hasSHT) {
    if (hasCO2 || hasPM) payload = payload + ",";
    TMP_RH result = ag.periodicFetchData();
    payload = payload + "\"atmp\":" + String(result.t) +   ",\"rhum\":" + String(result.rh);
    showTextRectangle(String(result.t), String(result.rh) + "%", false);
    delay(3000);
  }

  payload = payload + "}";

  currentMillis = millis();  //get the current "time" (actually the number of milliseconds since the program started)
  if (currentMillis - startMillis >= period)  //test whether the period has elapsed
  {

    //Serial.println(payload);
    String POSTURL = APIROOT + "/measures";

    WiFiClient client;

    if (client.connect(server, 8000)) {
      HTTPClient http;
      http.begin(client, POSTURL);

      http.addHeader("Content-Type", "application/json");

      int httpCode = http.POST(payload);
      //Serial.println(httpCode);
      //Serial.println(http.getString());
      http.end();
    }
    startMillis = currentMillis;  //IMPORTANT to save the start time of the current LED state.
  }

}

// DISPLAY
void showTextRectangle(String ln1, String ln2, boolean small) {
  display.clear();
  display.setTextAlignment(TEXT_ALIGN_LEFT);
  if (small) {
    display.setFont(ArialMT_Plain_16);
  } else {
    display.setFont(ArialMT_Plain_24);
  }
  display.drawString(32, 16, ln1);
  display.drawString(32, 36, ln2);
  display.display();
}

// Wifi Manager
void connectToWifi() {
  WiFiManager wifiManager;
  //WiFi.disconnect(); //to delete previous saved hotspot
  String HOTSPOT = "AIRGRADIENT-" + String(ESP.getChipId(), HEX);
  wifiManager.setTimeout(120);
  if (!wifiManager.autoConnect((const char*)HOTSPOT.c_str())) {
    Serial.println("failed to connect and hit timeout");
    delay(3000);
    ESP.restart();
    delay(5000);
  }

}
