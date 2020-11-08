use std::collections::hash_map::HashMap;
use std::io;

fn get_main_option() -> u32 {
    loop {
        println!("O que você deseja fazer?");
        println!("---");
        println!("1. Listar os servidores DNS");
        println!("2. Adicionar um novo servidor DNS");
        println!("3. Remover um servidor DNS");
        println!("4. Alterar um servidor DNS");
        println!("5. Sair");
        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Falha ao ler do console");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Opção inválida, tente novamente.");
                continue;
            }
        };
        if choice < 1 || choice > 5 {
            println!("Opção inválida, tente novamente.");
            continue;
        } else {
            return choice;
        }
    }
}

fn display_dns_servers(dns_servers: &HashMap<&str, (&str, &str)>) -> () {
    if dns_servers.len() > 0 {
        for (name, addresses) in dns_servers {
            println!("Servidor DNS \"{}\" com os endereços {:?}", name, addresses)
        }
    } else {
        println!("Não há servidores DNS cadastrados.")
    }
}

fn main() {
    //
    let mut dns_servers = HashMap::new();
    dns_servers.insert("Cloudflare", ("1.1.1.1", "1.0.0.1"));
    dns_servers.insert("Google", ("8.8.8.8", "8.8.4.4"));
    dns_servers.insert("OpenDNS", ("208.67.222.222", "208.67.220.220"));

    println!("Olá! Este programa gerencia seus servidores DNS.");

    loop {
        let option = get_main_option();
        match option {
            1 => display_dns_servers(&dns_servers),
            2 => { /*TODO: Adicionar um novo servidor DNS*/ }
            3 => { /*TODO: Remover um servidor DNS*/ }
            4 => { /*TODO: Alterar um servidor DNS*/ }
            _ => { /* TODO: Sair*/ }
        }
        if option == 5 {
            break;
        }
    }
}
