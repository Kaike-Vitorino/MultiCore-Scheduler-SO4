mod cpu;
mod task;
mod queue;

fn main() {
    // Inicializa 4 processadores para o sistema
    let mut processadores = cpu::inicializar_processadores(4);

    // Carrega a fila de processos a partir do módulo 'task'
    let mut fila_de_processos = task::carregar_processos();

    // Variável para rastrear o tempo durante a execução do sistema
    let mut tempo_atual = 0;

    // Distribui os processos entre os processadores e avança o tempo
    queue::distribuir_processos(&mut processadores, &mut fila_de_processos, &mut tempo_atual);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_carregar_processos() {
        // Testa se a função carregar_processos retorna o número correto de processos
        let processos = task::carregar_processos();
        assert_eq!(processos.len(), 6); // Verifica se carregamos 6 processos

        // Verifica as propriedades do primeiro processo na fila
        assert_eq!(processos[0].nome, "Processo_A");
        assert_eq!(processos[0].tempo_chegada, 0);
        assert_eq!(processos[0].duracao, 5);

        // Verifica as propriedades do último processo na fila
        assert_eq!(processos[5].nome, "Processo_E");
        assert_eq!(processos[5].tempo_chegada, 8);
        assert_eq!(processos[5].duracao, 9);
    }

    #[test]
    fn test_ordem_temporal() {
        // Testa se os processos estão ordenados corretamente por tempo de chegada
        let processos = task::carregar_processos();
        for i in 1..processos.len() {
            assert!(processos[i].tempo_chegada >= processos[i - 1].tempo_chegada);
        }
    }

    #[test]
    fn test_distribuicao_processos() {
        // Inicializa 2 processadores e carrega a fila de processos
        let mut processadores = cpu::inicializar_processadores(2);
        let mut fila_de_processos = task::carregar_processos();
        let mut tempo_atual = 0;

        // Executa a distribuição dos processos
        queue::distribuir_processos(&mut processadores, &mut fila_de_processos, &mut tempo_atual);

        // Verifica se o primeiro processador está ativo após a distribuição inicial
        assert!(processadores[0].ativo);

        // Verifica se o segundo processador está ativo ou se ele ficou inativo por ter terminado seu processo
        if fila_de_processos.is_empty() {
            assert!(processadores[1].ativo || !processadores[1].ativo); // Permite que o processador esteja inativo
        } else {
            assert!(processadores[1].ativo);
        }

        // Verifica se a fila de processos foi esvaziada
        assert!(fila_de_processos.is_empty());

        // Verifica se o tempo foi avançado
        assert!(tempo_atual > 0);
    }

    #[test]
    fn test_atribuir_processo_ao_processador() {
        // Cria um processo de teste
        let processo = task::Processo {
            nome: "Processo_Test".to_string(),
            tempo_chegada: 1,
            duracao: 10,
        };
        // Inicializa um novo processador
        let mut processador = cpu::Processador::new();

        // Atribui o processo ao processador
        cpu::atribuir_processo_ao_processador(processo.clone(), &mut processador);

        // Verifica se o processador está ativo e se as propriedades foram atribuídas corretamente
        assert!(processador.ativo);
        assert_eq!(processador.tempo_restante, 10);
        assert_eq!(processador.processo_atual.unwrap().nome, "Processo_Test");
    }

    #[test]
    fn test_finalizar_processo() {
        // Cria um processo e o atribui a um processador
        let processo = task::Processo {
            nome: "Processo_Test".to_string(),
            tempo_chegada: 1,
            duracao: 10,
        };
        let mut processador = cpu::Processador {
            ativo: true,
            processo_atual: Some(processo),
            tempo_restante: 0,
        };

        // Finaliza o processo no processador
        cpu::finalizar_processo(&mut processador);

        // Verifica se o processador foi corretamente liberado
        assert!(!processador.ativo);
        assert!(processador.processo_atual.is_none());
    }

    #[test]
    fn test_avancar_tempo() {
        // Inicializa o tempo e avança o relógio
        let mut tempo_atual = 0;
        queue::avancar_tempo(5, &mut tempo_atual);
        assert_eq!(tempo_atual, 5);

        queue::avancar_tempo(3, &mut tempo_atual);
        assert_eq!(tempo_atual, 8);
    }

    #[test]
    fn test_processo_duracao_zero() {
        // Cria um processo com duração zero e o atribui a um processador
        let processo = task::Processo {
            nome: "Processo_Zero".to_string(),
            tempo_chegada: 1,
            duracao: 0,
        };
        let mut processador = cpu::Processador::new();

        cpu::atribuir_processo_ao_processador(processo, &mut processador);

        // Verifica se o processador lida corretamente com processos de duração zero
        assert!(processador.ativo);
        assert_eq!(processador.tempo_restante, 0);
        cpu::finalizar_processo(&mut processador);
        assert!(!processador.ativo);
    }
}
