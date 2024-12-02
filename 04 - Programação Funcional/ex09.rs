fn busca_na_matriz(matriz: Vec<Vec<i32>>, linha: usize, coluna:usize) -> Option<i32>{
    if linha < matriz.len() && coluna < matriz[linha].len(){
        return Some(matriz[linha][coluna]);
    }
    None
}


fn main(){
    let matriz: Vec<Vec<i32>> = vec![
        [1, 2, 3].to_vec(),
        [4, 5, 6].to_vec(),
        [7, 8, 9].to_vec(),
    ];

    let linha:usize= 1;
    let coluna:usize = 2;


    let resultado = busca_na_matriz(matriz, linha, coluna);
    match resultado {
        Some(valor) => println!("{}", valor),
        None => println!("Indice invalido!")
    }
}