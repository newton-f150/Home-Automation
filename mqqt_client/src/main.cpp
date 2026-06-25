#include <Arduino.h>
#include <WiFi.h>
#include <PubSubClient.h>
#include <HTTPClient.h>

WiFiClient espClient;
PubSubClient client(espClient);

const char * ssid = "NezaLlc";
const char * password = "Neza@net11";
const char * mqtt_server = "100.84.169.93";


const int tempPin = 32;
const int presPin = 33;
const int airpin = 12;
const int lightPin = 14;

void setup_wifi(){
    delay(10);
    Serial.print("Connecting...");
    Serial.println(ssid);

    WiFi.begin(ssid,password);

    while(WiFi.status() != WL_CONNECTED){
        delay(500);
        Serial.print(".");
        // reconnect();
    }
    Serial.print("");
    Serial.print("\n Wifi Connected");
    
}

void reconnect() {
    while (!client.connected()) {
        Serial.println("Trying MQTT connection");

        if (client.connect("Esp32Client")) {
            Serial.println("Connected...");
        } else {
            Serial.print("Failed, rc=");
            Serial.println(client.state());
            delay(5000);
        }
    }
}


void setup() {
    Serial.begin(115200);

    setup_wifi();

    client.setServer(mqtt_server, 1883);

    reconnect();
}


void loop(){}