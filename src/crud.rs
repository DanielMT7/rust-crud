use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

use crate::produto::{Categoria, Produto};

const DB_FILE: &str = "produtos.txt";

pub fn criar_produto(produto: &Produto) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(DB_FILE)?;

    writeln!(
        &mut file,
        "{},{},{}",
        produto.nome, produto.preco, categoria_para_str(&produto.categoria)
    )?;

    Ok(())
}

pub fn ler_produtos() -> io::Result<Vec<Produto>> {
    let file = File::open(DB_FILE)?;
    let reader = BufReader::new(file);

    let mut produtos = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let campos: Vec<&str> = line.split(',').collect();

        let nome = campos[0].to_string();
        let preco: f64 = campos[1].parse().unwrap();
        let categoria = str_para_categoria(campos[2]);

        let produto = Produto {
            nome,
            preco,
            categoria,
        };

        produtos.push(produto);
    }

    Ok(produtos)
}

pub fn atualizar_produto(nome_antigo: &str, novo_produto: &Produto) -> io::Result<()> {
    let produtos = ler_produtos()?;
    let mut file = File::create(DB_FILE)?;

    for produto in produtos {
        if produto.nome == nome_antigo {
            writeln!(
                &mut file,
                "{},{},{}",
                novo_produto.nome,
                novo_produto.preco,
                categoria_para_str(&novo_produto.categoria)
            )?;
        } else {
            writeln!(
                &mut file,
                "{},{},{}",
                produto.nome,
                produto.preco,
                categoria_para_str(&produto.categoria)
            )?;
        }
    }

    Ok(())
}

pub fn deletar_produto(nome: &str) -> io::Result<()> {
    let produtos = ler_produtos()?;
    let mut file = File::create(DB_FILE)?;

    for produto in produtos {
        if produto.nome != nome {
            writeln!(
                &mut file,
                "{},{},{}",
                produto.nome,
                produto.preco,
                categoria_para_str(&produto.categoria)
            )?;
        }
    }

    Ok(())
}

fn categoria_para_str(categoria: &Categoria) -> &str {
    match categoria {
        Categoria::Eletronico => "Eletrônico",
        Categoria::Vestuario => "Vestuário",
        Categoria::Alimento => "Alimento",
    }
}

fn str_para_categoria(categoria_str: &str) -> Categoria {
    match categoria_str {
        "Eletrônico" => Categoria::Eletronico,
        "Vestuário" => Categoria::Vestuario,
        "Alimento" => Categoria::Alimento,
        _ => panic!("Categoria inválida"),
    }
}
