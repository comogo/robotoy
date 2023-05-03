#include <Radio.h>
#include <utils.h>

Radio::Radio(uint8_t id)
{
  mId = id;
  mSentPacketCounter = 0;
  mReceivedPacketCounter = 0;
  mLora = new LoRaClass();
}

Radio::~Radio()
{
  delete mLora;
}

void Radio::begin()
{
  SPI.begin(SCK, MISO, MOSI, SS);
  LoRa.setPins(SS, RST_LoRa, DIO0);
  mLora->begin(RADIO_FREQUENCY, true);
  mLora->setSpreadingFactor(7);
  mLora->setSignalBandwidth(500E3);
}

void Radio::sendPacket(Packet packet, bool reusePacketId)
{
  UUint32Byte_t packetId;
  UUint32Byte_t uPreviousPacketId;

  if (reusePacketId)
  {
    packetId.i = mLastPacketIdSent;
  }
  else
  {
    packetId.i = mSentPacketCounter;
  }

  uPreviousPacketId.i = mLastPacketIdSent;

  mLora->beginPacket();
  mLora->write(mId);                         // SENDER ID
  mLora->write(packet.address);              // RECEIVER ID
  mLora->write(packet.type);                 // PACKET TYPE
  mLora->write(packetId.b, 4);               // PACKET ID
  mLora->write(uPreviousPacketId.b, 4);      // PREVIOUS PACKET ID
  mLora->write(packet.payload, packet.size); // PAYLOAD
  mLora->endPacket();

  mLastPacketIdSent = packetId.i;

  mSentPacketCounter = increment(mSentPacketCounter);
}

bool Radio::receivePacket(Packet *data)
{
  int packetSize = mLora->parsePacket();

  if (packetSize <= 0)
  {
    return false;
  }

  UUint32Byte_t uPacketId;
  UUint32Byte_t uPreviousPacketId;

  uint8_t sender = (uint8_t)mLora->read();
  uint8_t receiver = (uint8_t)mLora->read();
  uint8_t type = (uint8_t)mLora->read();
  for (int i = 0; i < 4; i++)
  {
    uPacketId.b[i] = (uint8_t)mLora->read();
  }
  for (int i = 0; i < 4; i++)
  {
    uPreviousPacketId.b[i] = (uint8_t)mLora->read();
  }
  uint8_t payloadSize = packetSize - 3;
  uint8_t payload[payloadSize];

  for (int i = 0; i < payloadSize; i++)
  {
    payload[i] = (uint8_t)mLora->read();
  }

  if (receiver != mId)
  {
    return false;
  }

  // Discard already received packets
  if (uPacketId.i == mLastPacketIdReceived)
  {
    return false;
  }

  mLastPacketIdReceived = uPacketId.i;
  data->id = uPacketId.i;
  data->address = sender;
  data->type = type;
  data->size = payloadSize;
  data->previousPacketId = uPreviousPacketId.i;
  memcpy(data->payload, payload, payloadSize);

  mReceivedPacketCounter = increment(mReceivedPacketCounter);

  return true;
}

uint32_t Radio::getSentPacketCounter(){
  return mSentPacketCounter;
}

uint32_t Radio::getReceivedPacketCounter(){
  return mReceivedPacketCounter;
}

int Radio::getRSSI(){
  return mLora->packetRssi();
}

float Radio::getSNR(){
  return mLora->packetSnr();
}
