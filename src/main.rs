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

fn display_dns_servers(dns_servers: &HashMap<String, Vec<String>>) -> () {
    if dns_servers.len() > 0 {
        for (name, addresses) in dns_servers {
            println!("Servidor DNS \"{}\" com os endereços {:?}", name, addresses)
        }
    } else {
        println!("Não há servidores DNS cadastrados.")
    }
}

fn get_addresses_for_server(len: usize) -> Vec<String> {
    let mut addresses: Vec<String> = Vec::with_capacity(len);
    loop {
        let mut address = String::new();
        println!(
            "Digite o endereço do servidor {}",
            if addresses.len() > 0 {
                "secundário"
            } else {
                "primário"
            }
        );
        io::stdin()
            .read_line(&mut address)
            .expect("Falha ao ler do console");
        let address = address.trim();
        if address.len() == 0 {
            println!("Entrada inválida, tente novamente.");
            continue;
        }
        addresses.push(address.to_string());
        if addresses.len() == len {
            break;
        }
    }
    return addresses;
}

fn add_new_dns_server(dns_servers: &mut HashMap<String, Vec<String>>) -> () {
    loop {
        let mut name = String::new();

        println!("Digite o nome do provedor");
        io::stdin()
            .read_line(&mut name)
            .expect("Falha ao ler do console");
        if name.trim().len() == 0 {
            println!("Entrada inválida, tente novamente.");
            continue;
        }

        let addresses = get_addresses_for_server(2);
        dns_servers.insert(name.trim().to_string(), addresses);
        break;
    }
}

fn remove_dns_server(dns_servers: &mut HashMap<String, Vec<String>>) -> () {
    if dns_servers.len() == 0 {
        println!("Não existem servidores DNS cadastrados.");
        return;
    }
    loop {
        let mut name = String::new();
        println!("Digite o nome do provedor");
        io::stdin()
            .read_line(&mut name)
            .expect("Falha ao ler do console");
        if name.trim().len() == 0 {
            println!("Entrada inválida, tente novamente.");
            continue;
        }
        let name = name.trim();
        match dns_servers.remove(name) {
            Some(_) => {
                println!("Servidor {} removido com sucesso", &name);
                break;
            }
            None => {
                println!(
                    "Não existe servidor DNS cadastrado com este nome: {}. Tente novamente\n",
                    &name
                );
                continue;
            }
        }
    }
}

fn update_dns_server(dns_servers: &mut HashMap<String, Vec<String>>) -> () {
    loop {
        let mut name = String::new();
        println!("Digite o nome do provedor");
        io::stdin()
            .read_line(&mut name)
            .expect("Falha ao ler do console");
        if name.trim().len() == 0 {
            println!("Entrada inválida, tente novamente.");
            continue;
        }
        let name = name.trim();
        match dns_servers.get(name) {
            Some(_) => {
                let addresses = get_addresses_for_server(2);
                dns_servers.insert(name.to_string(), addresses);
                break;
            }
            None => {
                println!(
                    "Não existe servidor DNS cadastrado com este nome: {}. Tente novamente\n",
                    &name
                );
                continue;
            }
        }
    }
}

fn main() {
    //
    let mut dns_servers = HashMap::new();
    dns_servers.insert(
        "Cloudflare".to_string(),
        vec!["1.1.1.1".to_string(), "1.0.0.1".to_string()],
    );
    dns_servers.insert(
        "Google".to_string(),
        vec!["8.8.8.8".to_string(), "8.8.4.4".to_string()],
    );
    dns_servers.insert(
        "OpenDNS".to_string(),
        vec!["208.67.222.222".to_string(), "208.67.220.220".to_string()],
    );

    println!("Olá! Este programa gerencia seus servidores DNS.");

    loop {
        let option = get_main_option();
        match option {
            1 => display_dns_servers(&dns_servers),
            2 => add_new_dns_server(&mut dns_servers),
            3 => remove_dns_server(&mut dns_servers),
            4 => update_dns_server(&mut dns_servers),
            _ => {
                println!("Tchau!");
                break;
            }
        }
    }
}
