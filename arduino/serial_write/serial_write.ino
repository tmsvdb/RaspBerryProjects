const int DATA_PIN = 8;
const int LATCH_PIN = 9;
const int CLOCK_PIN = 10;

void setup() {
  Serial.begin(9600);  
  Serial.println("--- Start Serial Monitor SEND_RCVE ---");
  
  pinMode(DATA_PIN, OUTPUT);
  pinMode(LATCH_PIN, OUTPUT);
  pinMode(CLOCK_PIN, OUTPUT);

  
}

void loop() {

  digitalWrite(4, HIGH); 
  digitalWrite(5, HIGH); 
  digitalWrite(6, HIGH); 
  digitalWrite(7, HIGH); 
  
  digitalWrite(13, HIGH); 
  delay(500);
  digitalWrite(13, LOW); 
  delay(500);
  /*
  for (int counter = 0; counter < 256; counter++) {
      write(counter);
      delay(10);
  }*/
}

void write(byte b) {
  digitalWrite(LATCH_PIN, LOW);          //Pull latch LOW to start sending data
  shiftOut(DATA_PIN, CLOCK_PIN, MSBFIRST, b);         //Send the data
  digitalWrite(LATCH_PIN, HIGH);     
}
