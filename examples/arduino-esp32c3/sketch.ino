// ES8311 audio codec demo for the Wokwi web editor (Arduino, ESP32-C3).
//
// Configures the ES8311 Wokwi chip over raw I2C and prints the resulting register state in
// the same format as the Rust example in ../esp32c3. Wiring: SDA = GPIO4, SCL = GPIO5,
// CE = GND (address 0x18). The sketch does not drive I2S, so the codec produces no sound.

#include <Wire.h>

constexpr uint8_t ES8311_ADDRESS = 0x18;
constexpr uint8_t ES8311_CHIP_ID1 = 0x83;
constexpr uint8_t ES8311_CHIP_ID2 = 0x11;

constexpr uint8_t REG_RESET = 0x00;
constexpr uint8_t REG_CLK_MANAGER_01 = 0x01;
constexpr uint8_t REG_CLK_MANAGER_02 = 0x02;
constexpr uint8_t REG_CLK_MANAGER_03 = 0x03;
constexpr uint8_t REG_CLK_MANAGER_04 = 0x04;
constexpr uint8_t REG_CLK_MANAGER_05 = 0x05;
constexpr uint8_t REG_SDP_IN = 0x09;
constexpr uint8_t REG_SDP_OUT = 0x0A;
constexpr uint8_t REG_SYSTEM_0B = 0x0B;
constexpr uint8_t REG_SYSTEM_0C = 0x0C;
constexpr uint8_t REG_SYSTEM_0D = 0x0D;
constexpr uint8_t REG_SYSTEM_0E = 0x0E;
constexpr uint8_t REG_SYSTEM_10 = 0x10;
constexpr uint8_t REG_SYSTEM_11 = 0x11;
constexpr uint8_t REG_SYSTEM_12 = 0x12;
constexpr uint8_t REG_SYSTEM_13 = 0x13;
constexpr uint8_t REG_DAC_31 = 0x31;
constexpr uint8_t REG_DAC_32 = 0x32;
constexpr uint8_t REG_DAC_37 = 0x37;
constexpr uint8_t REG_GPIO_44 = 0x44;
constexpr uint8_t REG_GP_45 = 0x45;
constexpr uint8_t REG_CHIP_ID1 = 0xFD;
constexpr uint8_t REG_CHIP_ID2 = 0xFE;
constexpr uint8_t REG_CHIP_VER = 0xFF;

// DAC_31: DAC_DSMMUTE and DAC_DEMMUTE.
constexpr uint8_t DAC_MUTE = 0x60;

constexpr uint32_t RESET_DELAY_MS = 5;

// The volume limit of the Kode Dot, whose speaker amplifier adds gain.
constexpr uint8_t MAX_VOLUME = 0xA9;

constexpr int PIN_SDA = 4;
constexpr int PIN_SCL = 5;

struct Write {
  uint8_t reg;
  uint8_t value;
};

// Reset value of register 0x00: all blocks in reset, CSM off.
constexpr Write RESET[] = {{REG_RESET, 0x1F}};

// Safe init for 48 kHz, 16-bit I2S, MCLK = 256 × fs: clocks on, DAC muted at minimum volume,
// analog blocks down, then the state machine started.
constexpr Write CONFIGURE_48K[] = {
    {REG_GPIO_44, 0x08},         // no ADC-to-DAC loopback (written twice, as in the datasheet)
    {REG_GPIO_44, 0x08},
    {REG_CLK_MANAGER_01, 0x30},  // MCLK and BCLK on
    {REG_CLK_MANAGER_02, 0x00},  // prescaler
    {REG_CLK_MANAGER_05, 0x00},  // ADC/DAC clock dividers
    {REG_CLK_MANAGER_03, 0x10},  // ADC oversampling
    {REG_CLK_MANAGER_04, 0x10},  // DAC oversampling
    {REG_SYSTEM_0B, 0x00},       // power-up timing
    {REG_SYSTEM_0C, 0x00},
    {REG_SYSTEM_10, 0x1F},       // bias
    {REG_SYSTEM_11, 0x7F},       // VSEL
    {REG_SDP_IN, 0x0C},          // I2S, 16 bit
    {REG_SDP_OUT, 0x4C},         // I2S, 16 bit, muted
    {REG_SYSTEM_13, 0x10},       // HPSW
    {REG_DAC_31, DAC_MUTE},      // DAC muted
    {REG_DAC_32, 0x00},          // minimum volume
    {REG_DAC_37, 0x08},          // ramp, EQ bypassed
    {REG_SYSTEM_12, 0x02},       // DAC down
    {REG_SYSTEM_0E, 0x6A},       // ADC power reset value
    {REG_SYSTEM_0D, 0xFC},       // analog down
    {REG_GP_45, 0x00},
    {REG_RESET, 0x80},           // CSM on, out of reset
    {REG_CLK_MANAGER_01, 0x3F},  // all clocks on
};

constexpr Write DAC_POWER_UP[] = {{REG_SYSTEM_0D, 0x01}, {REG_SYSTEM_12, 0x00}};

constexpr Write STANDBY[] = {
    {REG_DAC_32, 0x00}, {REG_SYSTEM_0E, 0x7A}, {REG_SYSTEM_12, 0x02},
    {REG_SYSTEM_0D, 0xFC}, {REG_RESET, 0x1F}, {REG_CLK_MANAGER_01, 0x00},
};

uint8_t readRegister(uint8_t reg) {
  Wire.beginTransmission(ES8311_ADDRESS);
  Wire.write(reg);
  if (Wire.endTransmission(false) != 0 || Wire.requestFrom(int(ES8311_ADDRESS), 1) != 1) {
    return 0;
  }

  return Wire.read();
}

void writeRegister(uint8_t reg, uint8_t value) {
  Wire.beginTransmission(ES8311_ADDRESS);
  Wire.write(reg);
  Wire.write(value);
  Wire.endTransmission();
}

template <size_t N>
void writeAll(const Write (&writes)[N]) {
  for (const Write &w : writes) {
    writeRegister(w.reg, w.value);
  }
}

void setMute(bool mute) {
  uint8_t dac = readRegister(REG_DAC_31);
  writeRegister(REG_DAC_31, mute ? dac | DAC_MUTE : dac & ~DAC_MUTE);
}

void setup() {
  Serial.begin(115200);

  Wire.begin(PIN_SDA, PIN_SCL);
  Wire.setClock(400000);

  uint16_t chipId = uint16_t(readRegister(REG_CHIP_ID1)) << 8 | readRegister(REG_CHIP_ID2);
  if (chipId != (uint16_t(ES8311_CHIP_ID1) << 8 | ES8311_CHIP_ID2)) {
    Serial.printf("ES8311 not found (chip ID 0x%04X)\n", chipId);
    while (true) {
      delay(1000);
    }
  }

  uint8_t version = readRegister(REG_CHIP_VER);
  Serial.printf("ES8311 ok chip_id=0x%04X version=0x%02X\n", chipId, version);

  writeAll(RESET);
  delay(RESET_DELAY_MS);
  writeAll(CONFIGURE_48K);
  Serial.printf("init mute=0x%02X volume=0x%02X power=0x%02X dac=0x%02X routing=0x%02X\n",
                readRegister(REG_DAC_31), readRegister(REG_DAC_32), readRegister(REG_SYSTEM_0D),
                readRegister(REG_SYSTEM_12), readRegister(REG_GPIO_44));

  // Cap the volume at MAX_VOLUME, then request full scale.
  if (readRegister(REG_DAC_32) > MAX_VOLUME) {
    writeRegister(REG_DAC_32, MAX_VOLUME);
  }
  writeRegister(REG_DAC_32, min<uint8_t>(0xFF, MAX_VOLUME));
  Serial.printf("volume limit=0x%02X requested=0xFF written=0x%02X\n", MAX_VOLUME,
                readRegister(REG_DAC_32));

  writeAll(DAC_POWER_UP);
  setMute(false);
  Serial.printf("playing mute=0x%02X power=0x%02X dac=0x%02X\n", readRegister(REG_DAC_31),
                readRegister(REG_SYSTEM_0D), readRegister(REG_SYSTEM_12));

  setMute(true);
  writeAll(STANDBY);
  Serial.printf("standby reset=0x%02X clocks=0x%02X power=0x%02X\n", readRegister(REG_RESET),
                readRegister(REG_CLK_MANAGER_01), readRegister(REG_SYSTEM_0D));
}

void loop() {
  delay(1000);
}
