
import os
import ast
from datetime import datetime
import shutil
from pathlib import Path

# 🎨 ANSI Escape Codes para cores no terminal
RESET = '\033[0m'
BOLD = '\033[1m'

def fg(color_code):
    return f'\033[38;5;{color_code}m'

def bg(color_code):
    return f'\033[48;5;{color_code}m'

# 🎯 Paleta de cores - Inspirada no Tokyo Night Day (claro)
# Referência: tons suaves de azul, ciano, lilás e rosa

TEXT_PRIMARY = fg(236)       # Cinza escuro suave (texto base)
TEXT_SECONDARY = fg(245)     # Cinza médio (texto auxiliar)
ACCENT_BLUE = fg(68)         # Azul Tokyo
ACCENT_CYAN = fg(38)         # Ciano vibrante
ACCENT_PURPLE = fg(135)      # Roxo suave
ACCENT_PINK = fg(169)        # Rosa pastel
ACCENT_GREEN = fg(71)        # Verde suave
WARNING = fg(214)            # Amarelo Tokyo (alerta)
ERROR = fg(203)              # Vermelho suave (erro)
SUCCESS = fg(71)             # Verde (sucesso)

# 🎨 Cores adicionais
HIGHLIGHT = fg(117)          # Ciano mais claro

# Configurações
MATERIAS = ['Direito Constitucional', 'Português', 'Raciocínio Lógico', 'Direito Administrativo']
TIME_FILE = 'data/time'
BACKUP_DIR = 'backup'
Q_MATERIAS = 3


def materias_dia(indices):
    return [MATERIAS[i % len(MATERIAS)] for i in indices[:Q_MATERIAS]]


def atualizar_indice():
    if not os.path.exists(TIME_FILE):
        with open(TIME_FILE, 'w', encoding='utf-8') as f:
            f.write(str([0, 1]) + '\n')

    with open(TIME_FILE, 'r', encoding='utf-8') as f:
        linhas = f.readlines()
        indice = ast.literal_eval(linhas[0])

    for i in range(len(indice)):
        indice[i] = (indice[i] + Q_MATERIAS) % len(MATERIAS)

    with open(TIME_FILE, 'w', encoding='utf-8') as f:
        f.write(str(indice) + '\n')
        f.writelines(linhas[1:])

    return indice


def ponto_de_parada(materia):
    caminho = f'data/{materia}'
    if not os.path.exists(caminho):
        with open(caminho, 'w', encoding='utf-8') as f:
            f.write('Aula:0, Pág:0\n')
        return 'Aula:0, Pág:0'

    with open(caminho, 'r', encoding='utf-8') as f:
        linhas = f.readlines()
        return linhas[-1].strip() if linhas else 'Aula:0, Pág:0'


def fazer_backup():
    Path(BACKUP_DIR).mkdir(exist_ok=True)
    backup_path = os.path.join(BACKUP_DIR, f"time_backup_{datetime.now().strftime('%Y%m%d_%H%M%S')}.txt")
    shutil.copy2(TIME_FILE, backup_path)
    print(HIGHLIGHT + f"📦 Backup criado em: {backup_path}" + RESET)


def malfeito_feito(materias):
    os.system('clear')

    hoje = datetime.today().date()

    with open(TIME_FILE, 'r', encoding='utf-8') as f:
        linhas = f.readlines()

    if not linhas:
        print(ERROR + "⚠️  Arquivo de tempo vazio. Abortando." + RESET)
        return

    indice_str = linhas[0].strip()
    ultima_linha = linhas[-1].strip() if len(linhas) > 1 else ''

    if ultima_linha == str(hoje):
        print(WARNING + "✅ A data de hoje já foi registrada. Nenhuma ação será tomada." + RESET)
        return

    registros_validos = []
    alguma_estudada = False

    for materia in materias:
        aula = input(ACCENT_CYAN + f'📘 Em {materia}, parou em qual aula? ' + RESET)
        pag = input(ACCENT_CYAN + f'📄 E qual página? ' + RESET)

        if aula.isdigit() and pag.isdigit():
            aula, pag = int(aula), int(pag)
            if aula == 0 and pag == 0:
                print(WARNING + f"⚠️ {materia}: Nenhum progresso informado. Ignorado." + RESET)
                continue
            else:
                registros_validos.append((materia, aula, pag))
                alguma_estudada = True
        else:
            print(ERROR + '❌ Dados inválidos. Nada será registrado.' + RESET)
            return

    if not alguma_estudada:
        print(WARNING + "⚠️ Nenhuma matéria foi estudada. Nada será registrado." + RESET)
        return

    if any(aula == 0 and pag == 0 for _, aula, pag in registros_validos):
        print(WARNING + "⚠️ Nem todas as matérias foram estudadas. Reiniciando datas..." + RESET)
        fazer_backup()
        with open(TIME_FILE, 'w', encoding='utf-8') as f:
            f.write(indice_str + '\n')
            f.write(str(hoje) + '\n')
    else:
        with open(TIME_FILE, 'a', encoding='utf-8') as f:
            f.write(str(hoje) + '\n')
        print(SUCCESS + "🟢 Data de hoje adicionada ao final do arquivo." + RESET)
        atualizar_indice()

    for materia, aula, pag in registros_validos:
        caminho = f'data/{materia}'
        with open(caminho, 'a', encoding='utf-8') as f:
            f.write(f'\nAula:{aula}, Pág:{pag}')
        print(SUCCESS + f'✅ Progresso registrado para {materia}.' + RESET)


def revisao(materia, dias_atras):
    caminho = f'data/{materia}'
    with open(caminho, 'r', encoding='utf-8') as f:
        linhas = f.readlines()

    if len(linhas) < dias_atras + 1:
        return [None, None]

    return [linhas[-(dias_atras + 1)].strip(), linhas[-(dias_atras)].strip()]


def mostrar_revisao(titulo_colorido, revisao):
    print(titulo_colorido)
    print(TEXT_SECONDARY + f'📌 {"De:".ljust(10)} {revisao[0]}' + RESET)
    print(SUCCESS + f'➡️  {"Para:".ljust(10)} {revisao[1]}' + RESET)


def main():
    os.system('clear')

    if os.path.exists(TIME_FILE):
        with open(TIME_FILE, 'r', encoding='utf-8') as f:
            linhas = f.readlines()
        indice = ast.literal_eval(linhas[0]) if linhas else [0, 1]
    else:
        indice = [0, 1]

    materias = materias_dia(indice)

    print(ACCENT_PURPLE + BOLD + "\n📚 MATÉRIAS DO DIA:" + RESET)

    for materia in materias:
        print(ACCENT_BLUE + BOLD + f'\n📘 {materia.upper()}' + RESET)
        print(WARNING + f'📍 Último ponto de parada: {ponto_de_parada(materia)}' + RESET)

        rev_ontem = revisao(materia, 1)
        if rev_ontem[0]:
            mostrar_revisao(ACCENT_CYAN + BOLD + '\n🔁 REVISÃO - ONTEM' + RESET, rev_ontem)

        rev_2_sem = revisao(materia, 14)
        if rev_2_sem[0]:
            mostrar_revisao(ACCENT_PINK + BOLD + '\n⏪ REVISÃO - 2 SEMANAS ATRÁS' + RESET, rev_2_sem)

    resposta = input(ACCENT_PINK + '\n🤔 Já estudou hoje? [s/n]: ' + RESET).strip().lower()
    if resposta in ['s', 'sim', 'yes', 'y', '1']:
        malfeito_feito(materias)
    else:
        print(WARNING + "👍 Ok, lembre-se de estudar mais tarde!" + RESET)


if __name__ == '__main__':
    main()
