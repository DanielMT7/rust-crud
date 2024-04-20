mod produto;
mod crud;

use std::io;

use produto::{Categoria, Produto};

fn main() {
    loop {
        println!("Escolha uma opção:");
        println!("1. Criar produto");
        println!("2. Ler produtos");
        println!("3. Atualizar produto");
        println!("4. Deletar produto");
        println!("5. Sair");

        let mut opcao = String::new();
        io::stdin().read_line(&mut opcao).expect("Falha ao ler entrada");

        match opcao.trim().parse() {
            Ok(1) => criar_produto(),
            Ok(2) => ler_produtos(),
            Ok(3) => atualizar_produto(),
            Ok(4) => deletar_produto(),
            Ok(5) => break,
            _ => println!("Opção inválida"),
        }
    }
}

fn criar_produto() {
    println!("Digite o nome do produto:");
    let nome = ler_entrada();

    println!("Digite o preço do produto:");
    let preco: f64 = ler_entrada().parse().expect("Preço inválido");

    println!("Escolha a categoria do produto:");
    println!("1. Eletrônico");
    println!("2. Vestuário");
    println!("3. Alimento");

    let categoria = match ler_entrada().trim().parse() {
        Ok(1) => Categoria::Eletronico,
        Ok(2) => Categoria::Vestuario,
        Ok(3) => Categoria::Alimento,
        _ => {
            println!("Opção inválida, categoria definida como Eletrônico");
            Categoria::Eletronico
        }
    };

    let produto = Produto {
        nome,
        preco,
        categoria,
    };

    if let Err(e) = crud::criar_produto(&produto) {
        println!("Erro ao criar produto: {}", e);
    } else {
        println!("Produto criado com sucesso!");
    }
}

fn ler_produtos() {
    match crud::ler_produtos() {
        Ok(produtos) => {
            if produtos.is_empty() {
                println!("Nenhum produto encontrado");
            } else {
                println!("Lista de produtos:");
                for produto in produtos {
                    println!(
                        "{} - R$ {:.2} - Categoria: {:?}",
                        produto.nome, produto.preco, produto.categoria
                    );
                }
            }
        }
        Err(e) => println!("Erro ao ler produtos: {}", e),
    }
}

fn atualizar_produto() {
    println!("Digite o nome do produto que deseja atualizar:");
    let nome_antigo = ler_entrada();

    println!("Digite o novo nome do produto:");
    let nome_novo = ler_entrada();

    println!("Digite o novo preço do produto:");
    let preco_novo: f64 = ler_entrada().parse().expect("Preço inválido");

    println!("Escolha a nova categoria do produto:");
    println!("1. Eletrônico");
    println!("2. Vestuário");
    println!("3. Alimento");

    let categoria_nova = match ler_entrada().trim().parse() {
        Ok(1) => Categoria::Eletronico,
        Ok(2) => Categoria::Vestuario,
        Ok(3) => Categoria::Alimento,
        _ => {
            println!("Opção inválida, categoria definida como Eletrônico");
            Categoria::Eletronico
        }
    };

    let novo_produto = Produto {
        nome: nome_novo,
        preco: preco_novo,
        categoria: categoria_nova,
    };

    if let Err(e) = crud::atualizar_produto(&nome_antigo, &novo_produto) {
        println!("Erro ao atualizar produto: {}", e);
    } else {
        println!("Produto atualizado com sucesso!");
    }
}

fn deletar_produto() {
    println!("Digite o nome do produto que deseja deletar:");
    let nome = ler_entrada();

    if let Err(e) = crud::deletar_produto(&nome) {
        println!("Erro ao deletar produto: {}", e);
    } else {
        println!("Produto deletado com sucesso!");
    }
}

fn ler_entrada() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Falha ao ler entrada");
    input.trim().to_string()
}
