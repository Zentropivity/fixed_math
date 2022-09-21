use fixed::types::U6F122;

pub const ATAN_2P_DEG: [U6F122; 129] = [
    U6F122::from_le_bytes([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 180]),
    U6F122::from_le_bytes([
        253, 233, 170, 62, 143, 108, 194, 166, 63, 15, 119, 152, 198, 156, 66, 106,
    ]),
    U6F122::from_le_bytes([
        157, 144, 94, 115, 80, 59, 222, 7, 222, 91, 192, 245, 1, 29, 37, 56,
    ]),
    U6F122::from_le_bytes([
        246, 168, 9, 143, 150, 34, 151, 173, 218, 130, 254, 39, 73, 4, 128, 28,
    ]),
    U6F122::from_le_bytes([
        71, 55, 163, 42, 102, 128, 123, 188, 115, 103, 178, 48, 153, 42, 78, 14,
    ]),
    U6F122::from_le_bytes([
        168, 174, 15, 160, 99, 197, 48, 231, 228, 169, 184, 148, 83, 222, 40, 7,
    ]),
    U6F122::from_le_bytes([
        218, 123, 118, 248, 82, 244, 55, 139, 169, 214, 97, 198, 106, 168, 148, 3,
    ]),
    U6F122::from_le_bytes([
        140, 168, 127, 36, 53, 6, 137, 83, 22, 247, 73, 132, 94, 91, 202, 1,
    ]),
    U6F122::from_le_bytes([
        69, 213, 208, 168, 174, 80, 178, 76, 21, 74, 86, 110, 148, 46, 229, 0,
    ]),
    U6F122::from_le_bytes([
        117, 218, 3, 156, 94, 47, 197, 113, 25, 195, 241, 220, 102, 151, 114, 0,
    ]),
    U6F122::from_le_bytes([
        14, 5, 49, 250, 117, 201, 189, 40, 246, 184, 51, 3, 183, 75, 57, 0,
    ]),
    U6F122::from_le_bytes([
        127, 134, 161, 51, 20, 70, 98, 138, 133, 71, 49, 244, 219, 165, 28, 0,
    ]),
    U6F122::from_le_bytes([
        123, 140, 103, 4, 59, 120, 72, 254, 164, 145, 107, 8, 238, 82, 14, 0,
    ]),
    U6F122::from_le_bytes([
        213, 122, 237, 47, 94, 11, 154, 205, 146, 38, 0, 6, 119, 41, 7, 0,
    ]),
    U6F122::from_le_bytes([
        208, 31, 250, 166, 22, 87, 214, 144, 1, 95, 57, 131, 187, 148, 3, 0,
    ]),
    U6F122::from_le_bytes([
        175, 113, 113, 246, 116, 42, 174, 206, 247, 216, 163, 193, 93, 202, 1, 0,
    ]),
    U6F122::from_le_bytes([
        229, 119, 138, 150, 189, 131, 39, 200, 170, 209, 210, 224, 46, 229, 0, 0,
    ]),
    U6F122::from_le_bytes([
        145, 226, 233, 93, 36, 16, 46, 64, 123, 133, 105, 112, 151, 114, 0, 0,
    ]),
    U6F122::from_le_bytes([
        61, 95, 195, 138, 222, 83, 154, 91, 82, 198, 52, 184, 75, 57, 0, 0,
    ]),
    U6F122::from_le_bytes([
        49, 76, 39, 238, 120, 147, 61, 197, 155, 99, 26, 220, 165, 28, 0, 0,
    ]),
    U6F122::from_le_bytes([
        34, 184, 22, 45, 238, 214, 140, 53, 220, 49, 13, 238, 82, 14, 0, 0,
    ]),
    U6F122::from_le_bytes([
        70, 17, 83, 81, 29, 45, 36, 229, 239, 152, 6, 119, 41, 7, 0, 0,
    ]),
    U6F122::from_le_bytes([
        225, 185, 34, 112, 195, 206, 221, 43, 120, 76, 131, 187, 148, 3, 0, 0,
    ]),
    U6F122::from_le_bytes([
        237, 132, 1, 81, 104, 94, 24, 29, 60, 166, 193, 93, 202, 1, 0, 0,
    ]),
    U6F122::from_le_bytes([
        133, 207, 158, 251, 20, 94, 113, 15, 30, 211, 224, 46, 229, 0, 0, 0,
    ]),
    U6F122::from_le_bytes([
        165, 41, 51, 152, 230, 84, 213, 7, 143, 105, 112, 151, 114, 0, 0, 0,
    ]),
    U6F122::from_le_bytes([
        17, 13, 102, 207, 46, 63, 238, 131, 199, 52, 184, 75, 57, 0, 0, 0,
    ]),
    U6F122::from_le_bytes([
        144, 149, 28, 216, 46, 146, 247, 193, 99, 26, 220, 165, 28, 0, 0, 0,
    ]),
    U6F122::from_le_bytes([
        169, 124, 27, 90, 106, 215, 251, 224, 49, 13, 238, 82, 14, 0, 0, 0,
    ]),
    U6F122::from_le_bytes([
        145, 100, 207, 138, 127, 237, 125, 240, 152, 6, 119, 41, 7, 0, 0, 0,
    ]),
    U6F122::from_le_bytes([
        16, 231, 31, 17, 249, 246, 62, 120, 76, 131, 187, 148, 3, 0, 0, 0,
    ]),
    U6F122::from_le_bytes([
        33, 250, 6, 178, 131, 123, 31, 60, 166, 193, 93, 202, 1, 0, 0, 0,
    ]),
    U6F122::from_le_bytes([
        228, 93, 50, 190, 194, 189, 15, 30, 211, 224, 46, 229, 0, 0, 0, 0,
    ]),
    U6F122::from_le_bytes([
        12, 11, 191, 123, 225, 222, 7, 143, 105, 112, 151, 114, 0, 0, 0, 0,
    ]),
    U6F122::from_le_bytes([
        9, 65, 116, 193, 112, 239, 131, 199, 52, 184, 75, 57, 0, 0, 0, 0,
    ]),
    U6F122::from_le_bytes([
        245, 183, 44, 97, 184, 247, 193, 99, 26, 220, 165, 28, 0, 0, 0, 0,
    ]),
    U6F122::from_le_bytes([
        233, 174, 164, 48, 220, 251, 224, 49, 13, 238, 82, 14, 0, 0, 0, 0,
    ]),
    U6F122::from_le_bytes([
        210, 33, 84, 24, 238, 125, 240, 152, 6, 119, 41, 7, 0, 0, 0, 0,
    ]),
    U6F122::from_le_bytes([
        53, 74, 42, 12, 247, 62, 120, 76, 131, 187, 148, 3, 0, 0, 0, 0,
    ]),
    U6F122::from_le_bytes([
        68, 44, 21, 134, 123, 31, 60, 166, 193, 93, 202, 1, 0, 0, 0, 0,
    ]),
    U6F122::from_le_bytes([
        7, 151, 10, 195, 189, 15, 30, 211, 224, 46, 229, 0, 0, 0, 0, 0,
    ]),
    U6F122::from_le_bytes([
        160, 75, 133, 225, 222, 7, 143, 105, 112, 151, 114, 0, 0, 0, 0, 0,
    ]),
    U6F122::from_le_bytes([
        212, 165, 194, 112, 239, 131, 199, 52, 184, 75, 57, 0, 0, 0, 0, 0,
    ]),
    U6F122::from_le_bytes([
        234, 82, 97, 184, 247, 193, 99, 26, 220, 165, 28, 0, 0, 0, 0, 0,
    ]),
    U6F122::from_le_bytes([
        117, 169, 48, 220, 251, 224, 49, 13, 238, 82, 14, 0, 0, 0, 0, 0,
    ]),
    U6F122::from_le_bytes([
        187, 84, 24, 238, 125, 240, 152, 6, 119, 41, 7, 0, 0, 0, 0, 0,
    ]),
    U6F122::from_le_bytes([
        93, 42, 12, 247, 62, 120, 76, 131, 187, 148, 3, 0, 0, 0, 0, 0,
    ]),
    U6F122::from_le_bytes([
        47, 21, 134, 123, 31, 60, 166, 193, 93, 202, 1, 0, 0, 0, 0, 0,
    ]),
    U6F122::from_le_bytes([
        151, 10, 195, 189, 15, 30, 211, 224, 46, 229, 0, 0, 0, 0, 0, 0,
    ]),
    U6F122::from_le_bytes([
        76, 133, 225, 222, 7, 143, 105, 112, 151, 114, 0, 0, 0, 0, 0, 0,
    ]),
    U6F122::from_le_bytes([
        166, 194, 112, 239, 131, 199, 52, 184, 75, 57, 0, 0, 0, 0, 0, 0,
    ]),
    U6F122::from_le_bytes([
        83, 97, 184, 247, 193, 99, 26, 220, 165, 28, 0, 0, 0, 0, 0, 0,
    ]),
    U6F122::from_le_bytes([
        169, 48, 220, 251, 224, 49, 13, 238, 82, 14, 0, 0, 0, 0, 0, 0,
    ]),
    U6F122::from_le_bytes([85, 24, 238, 125, 240, 152, 6, 119, 41, 7, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([42, 12, 247, 62, 120, 76, 131, 187, 148, 3, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([21, 134, 123, 31, 60, 166, 193, 93, 202, 1, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([11, 195, 189, 15, 30, 211, 224, 46, 229, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([
        133, 225, 222, 7, 143, 105, 112, 151, 114, 0, 0, 0, 0, 0, 0, 0,
    ]),
    U6F122::from_le_bytes([
        195, 112, 239, 131, 199, 52, 184, 75, 57, 0, 0, 0, 0, 0, 0, 0,
    ]),
    U6F122::from_le_bytes([97, 184, 247, 193, 99, 26, 220, 165, 28, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([49, 220, 251, 224, 49, 13, 238, 82, 14, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([24, 238, 125, 240, 152, 6, 119, 41, 7, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([12, 247, 62, 120, 76, 131, 187, 148, 3, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([134, 123, 31, 60, 166, 193, 93, 202, 1, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([195, 189, 15, 30, 211, 224, 46, 229, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([226, 222, 7, 143, 105, 112, 151, 114, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([113, 239, 131, 199, 52, 184, 75, 57, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([184, 247, 193, 99, 26, 220, 165, 28, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([220, 251, 224, 49, 13, 238, 82, 14, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([238, 125, 240, 152, 6, 119, 41, 7, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([247, 62, 120, 76, 131, 187, 148, 3, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([124, 31, 60, 166, 193, 93, 202, 1, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([190, 15, 30, 211, 224, 46, 229, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([223, 7, 143, 105, 112, 151, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([239, 131, 199, 52, 184, 75, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([248, 193, 99, 26, 220, 165, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([252, 224, 49, 13, 238, 82, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([126, 240, 152, 6, 119, 41, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([63, 120, 76, 131, 187, 148, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([31, 60, 166, 193, 93, 202, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([16, 30, 211, 224, 46, 229, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([8, 143, 105, 112, 151, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([132, 199, 52, 184, 75, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([194, 99, 26, 220, 165, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([225, 49, 13, 238, 82, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([240, 152, 6, 119, 41, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([120, 76, 131, 187, 148, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([60, 166, 193, 93, 202, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([30, 211, 224, 46, 229, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([143, 105, 112, 151, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([200, 52, 184, 75, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([100, 26, 220, 165, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([50, 13, 238, 82, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([153, 6, 119, 41, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([76, 131, 187, 148, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([166, 193, 93, 202, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([211, 224, 46, 229, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([106, 112, 151, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([53, 184, 75, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([26, 220, 165, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([13, 238, 82, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([7, 119, 41, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([131, 187, 148, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([194, 93, 202, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([225, 46, 229, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([112, 151, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([184, 75, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([220, 165, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([238, 82, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([119, 41, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([188, 148, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([94, 202, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([47, 229, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([151, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([76, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([166, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([83, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([41, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([149, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([202, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([229, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    U6F122::from_le_bytes([1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
];
