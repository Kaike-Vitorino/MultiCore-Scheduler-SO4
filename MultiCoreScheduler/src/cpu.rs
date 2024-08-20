// A struct Processador representa um processador no sistema, capaz de executar um processo.
// Ela inclui informações sobre se o processador está ativo, o processo atual que está executando,
// e o tempo restante para a conclusão do processo.
#[derive(Clone, Debug)] // Deriva os traits Clone e Debug para Processador
pub struct Processador {
    pub ativo: bool, // Indica se o processador está ativo
    pub processo_atual: Option<crate::task::Processo>, // O processo atualmente atribuído ao processador
    pub tempo_restante: usize, // O tempo restante para completar o processo atual
}

// Implementa métodos para a struct Processador
impl Processador {
    // Função para criar um novo processador inativo, sem processo atribuído
    pub fn new() -> Self {
        Processador {
            ativo: false,
            processo_atual: None,
            tempo_restante: 0,
        }
    }
}

// Função para inicializar um vetor de processadores
// A função retorna um vetor de 'n' processadores inativos.
pub(crate) fn inicializar_processadores(n: usize) -> Vec<Processador> {
    vec![Processador::new(); n] // Cria um vetor com 'n' processadores novos
}

// Atribui um processo ao processador, ativando-o e definindo o tempo restante
pub fn atribuir_processo_ao_processador(
    processo: crate::task::Processo, // O processo a ser atribuído
    processador: &mut Processador,    // O processador que receberá o processo
) {
    processador.ativo = true; // Marca o processador como ativo
    processador.processo_atual = Some(processo); // Atribui o processo ao processador
    // Define o tempo restante com base na duração do processo
    processador.tempo_restante = processador.processo_atual.as_ref().unwrap().duracao;
}

// Finaliza o processo atual no processador, liberando-o
pub fn finalizar_processo(processador: &mut Processador) {
    // Imprime uma mensagem indicando qual processo foi finalizado
    println!(
        "Finalizando processo: {}",
        processador.processo_atual.as_ref().unwrap().nome
    );
    processador.ativo = false; // Marca o processador como inativo
    processador.processo_atual = None; // Remove o processo do processador
}
