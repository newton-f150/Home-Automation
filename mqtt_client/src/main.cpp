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

            // Client Subsribe to a topic from the broker
            client.subscribe("esp/cmd");
        } else {
            Serial.print("Failed, rc=");
            Serial.println(client.state());
            delay(5000);
        }
    }
}

void callback(char * topic, byte * payload,unsigned int length){
    String message;

    for (int i=0;i<length;i++){
        message+=(char)payload[i];
    }

    Serial.print(message);
   if(String(topic) == "esp/cmd"){
    Serial.print("Received Message: " +message);
    parseStr(message);
   }
}

// void parseStr(const String& str){
//     int commaIndex = str.indexOf(",");

//     String sensorName = str.substring(0,commandIndex);
//     int pinState = str.substring(command + 1).toInt();
// }


void setup() {
    Serial.begin(115200);

    setup_wifi();

    client.setServer(mqtt_server, 1883);

    reconnect();
}


void loop(){}