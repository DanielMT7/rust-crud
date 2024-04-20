// src/produto.rs

#[derive(Debug)]
pub enum Categoria {
  Eletronico,
  Vestuario,
  Alimento,
}

pub struct Produto {
  pub nome: String,
  pub preco: f64,
  pub categoria: Categoria,
}
