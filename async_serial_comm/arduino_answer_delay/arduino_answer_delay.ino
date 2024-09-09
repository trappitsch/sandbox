/* 
  This is a very simple program that listens for an SCPI command called "DELAY <time>" 
  and responds with "ACK" after the specified delay time (in seconds).

  It uses the `Vrekrer_scpi_parser` library to parse the incoming SCPI commands.

  The idea of this simply Arduino test script is to develop asynchronous serial querying 
  for `instrumentkit`.
*/

#include <Arduino.h>
#include <Vrekrer_scpi_parser.h>

SCPI_Parser my_instrument;

void setup() {
    my_instrument.RegisterCommand(F("DELAY"), &SetDelay);
    Serial.begin(9600);
}

void loop() {
    my_instrument.ProcessInput(Serial, "\n");
}

void SetDelay(SCPI_C commands, SCPI_P parameters, Stream& interface) {
    if (parameters.Size() > 0) {
        int delay_time = String(parameters[0]).toInt();
        delay(delay_time * 1000);
        interface.println(F("ACK-SECOND"));
    }
}
 
