// A struct Processo representa uma tarefa que será executada pelos processadores.
// Inclui o nome da tarefa, o tempo de chegada e a duração da execução.
#[derive(Clone, Debug)] // Deriva os traits Clone e Debug para Processo
pub struct Processo {
    pub nome: String, // Nome do processo
    pub tempo_chegada: usize, // Tempo de chegada do processo ao sistema
    pub duracao: usize, // Duração necessária para completar o processo
}

// Função que carrega uma lista de processos predefinidos.
// Retorna um vetor contendo vários processos com diferentes tempos de chegada e durações.
pub fn carregar_processos() -> Vec<Processo> {
    vec![
        Processo {
            nome: "Processo_A".to_string(),
            tempo_chegada: 0,
            duracao: 5,
        },
        Processo {
            nome: "Processo_B".to_string(),
            tempo_chegada: 2,
            duracao: 3,
        },
        Processo {
            nome: "Processo_C".to_string(),
            tempo_chegada: 4,
            duracao: 7,
        },
        Processo {
            nome: "Processo_D".to_string(),
            tempo_chegada: 5,
            duracao: 1,
        },
        Processo {
            nome: "Processo_F".to_string(),
            tempo_chegada: 6,
            duracao: 4,
        },
        Processo {
            nome: "Processo_E".to_string(),
            tempo_chegada: 8,
            duracao: 9,
        },
    ]
}

// Função que ordena a fila de processos com base no tempo de chegada.
// Ordena a fila em ordem crescente de tempo de chegada, para que os processos que chegam primeiro sejam executados primeiro.
pub fn ordenar_fila_por_tempo_de_chegada(fila_de_processos: &mut Vec<Processo>) {
    fila_de_processos.sort_by_key(|p| p.tempo_chegada); // Usa sort_by_key para ordenar por tempo_chegada
}

// Função que obtém e remove o próximo processo na fila de processos.
// Retorna o processo que está na frente da fila, removendo-o do vetor.
pub fn obter_proximo_processo(fila_de_processos: &mut Vec<Processo>) -> Processo {
    fila_de_processos.remove(0) // Remove e retorna o primeiro processo da fila
}
