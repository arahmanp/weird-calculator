pub fn help(cmd: &[&str]) {
    if cmd.len() != 1 {
        println!("Perintah \'help\' tidak menerima argumen!");
        return;
    }
    
    println!("Perintah yang tersedia:");
    println!("");
    println!("    help            menampilkan semua perintah yang tersedia");
    println!("    about           menampilkan informasi program");
    println!("    add a b         menjumlahkan bilangan a dan b");
    println!("    clear           membersihkan layar terminal");
    println!("    exit            keluar program");
    println!("");
}