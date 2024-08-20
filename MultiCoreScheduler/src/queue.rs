// Função responsável por distribuir processos entre os processadores e gerenciar o tempo de execução.
// Esta função executa um loop até que todos os processos na fila tenham sido distribuídos e concluídos.
pub fn distribuir_processos(
    processadores: &mut [crate::cpu::Processador], // Array de processadores disponíveis
    fila_de_processos: &mut Vec<crate::task::Processo>, // Fila de processos a serem distribuídos
    tempo_atual: &mut usize, // Referência ao tempo atual do sistema
) {
    // Loop principal que continua até que a fila de processos esteja vazia
    while !fila_de_processos.is_empty() {
        // Ordena a fila de processos com base no tempo de chegada
        crate::task::ordenar_fila_por_tempo_de_chegada(fila_de_processos);

        // Exibe o tempo atual e a fila de processos
        println!("Tempo: {}", tempo_atual);
        println!("Fila de processos: {:?}", fila_de_processos);

        // Atribui processos aos processadores que estão inativos
        for (i, processador) in processadores.iter_mut().enumerate() {
            if !processador.ativo && !fila_de_processos.is_empty() {
                let processo = crate::task::obter_proximo_processo(fila_de_processos);
                println!("Atribuindo processo ao processador {}: {:?}", i, processo);
                crate::cpu::atribuir_processo_ao_processador(processo, processador);
            }
        }

        // Avança o tempo do sistema em uma unidade
        avancar_tempo(1, tempo_atual);

        // Atualiza o estado dos processadores após avançar o tempo
        for (i, processador) in processadores.iter_mut().enumerate() {
            if processador.ativo {
                // Reduz o tempo restante para o processo atual no processador
                processador.tempo_restante -= 1;
                println!(
                    "Processador {} está executando um processo, tempo restante: {}",
                    i, processador.tempo_restante
                );
                // Se o processo atual foi concluído, finaliza-o e libera o processador
                if processador.tempo_restante == 0 {
                    println!("Processo no processador {} finalizado.", i);
                    crate::cpu::finalizar_processo(processador);
                }
            }
        }

        // Imprime uma linha de separação para indicar o final de um ciclo de tempo
        println!("---");
    }
}

// Função responsável por avançar o tempo do sistema.
// A função incrementa o tempo atual pelo número de unidades especificado.
pub fn avancar_tempo(unidades: usize, tempo_atual: &mut usize) {
    *tempo_atual += unidades; // Incrementa o tempo atual pelo valor de 'unidades'
}
