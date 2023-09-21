const hexString =
  "f43178a3593aaf6e31362e6a887bb46536c32a0b4294fc7ba18128ed48198ed1";

function hexStringToByteArray(hex: string): number[] {
  let bytes = [];
  for (let i = 0; i < hex.length; i += 2) {
    bytes.push(parseInt(hex.substring(i, i + 2), 16));
  }
  return bytes;
}

const byteArray = hexStringToByteArray(hexString);
console.log(byteArray);
