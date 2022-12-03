int m1=5;
int m1=6;
int m1=7;
int m1=8;
int m1=9;
int m1=10;

void setup()
{
    pinMode(m1, OUTPUT);
    pinMode(m2, OUTPUT);
    pinMode(m3, OUTPUT);
    pinMode(m4, OUTPUT);
    pinMode(m5, OUTPUT);
    pinMode(m6, OUTPUT);
    pinMode(m7, OUTPUT);
    pinMode(m8, OUTPUT);
    pinMode(m9, OUTPUT);
    pinMode(m10, OUTPUT);
}

void loop()
{
    digitalWrite(m1, HIGH);
    delay(1000);
    digitalWrite(m1, LOW);
}