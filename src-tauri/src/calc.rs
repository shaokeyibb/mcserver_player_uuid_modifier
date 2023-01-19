/*
public static UUID nameUUIDFromBytes(byte[] name) {
        MessageDigest md;
        try {
            md = MessageDigest.getInstance("MD5");
        } catch (NoSuchAlgorithmException nsae) {
            throw new InternalError("MD5 not supported", nsae);
        }
        byte[] md5Bytes = md.digest(name);
        md5Bytes[6]  &= 0x0f;  
        md5Bytes[6]  |= 0x30;  
        md5Bytes[8]  &= 0x3f;  
        md5Bytes[8]  |= 0x80;  
        return new UUID(md5Bytes);
    }
 */
#[tauri::command]
pub fn name_uuid_from_bytes(name: Vec<u8>) -> String{
    let mut md5_bytes =  md5::compute(name).0;
    md5_bytes[6] &= 0x0f;
    md5_bytes[6] |= 0x30;
    md5_bytes[8] &= 0x3f;
    md5_bytes[8] |= 0x80;
    let uuid = md5_bytes.into_iter()
    .map(|x| format!("{:02x}", x))
    .collect::<Vec<String>>().join("");
    String::from("") + &uuid[..8] + "-" + &uuid[8..12] + "-" + &uuid[12..16] + "-" + &uuid[16..20] + "-" + &uuid[20..]
}