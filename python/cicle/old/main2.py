import os
import ast
from datetime import datetime, timedelta
import shutil
from pathlib import Path

try:
    from colorama import init, Fore, Style
    init(autoreset=True)
except ImportError:
    class Fore:
        GREEN = YELLOW = RED = CYAN = BLUE = MAGENTA = WHITE = RESET = ''
        LIGHTGREEN_EX = LIGHTCYAN_EX = LIGHTYELLOW_EX = LIGHTMAGENTA_EX = ''
    class Style:
        BRIGHT = RESET_ALL = ''

# Configurações
MATERIAS = ['Direito Constitucional', 'Português','Raciocínio Lógico', 'Direito Administrativo']
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
    print(Fore.LIGHTCYAN_EX + f"📦 Backup criado em: {backup_path}")


def malfeito_feito(materias):
    try:
        os.system('cls' if os.name == 'nt' else 'clear')
    except:
        pass

    hoje = datetime.today().date()
    ontem = hoje - timedelta(days=1)

    with open(TIME_FILE, 'r', encoding='utf-8') as f:
        linhas = f.readlines()

    if not linhas:
        print(Fore.RED + "⚠️  Arquivo de tempo vazio. Abortando.")
        return

    indice_str = linhas[0].strip()
    ultima_linha = linhas[-1].strip() if len(linhas) > 1 else ''

    if ultima_linha == str(hoje):
        print(Fore.LIGHTYELLOW_EX + "✅ A data de hoje já foi registrada. Nenhuma ação será tomada.")
        return

    registros_validos = []
    alguma_estudada = False

    for materia in materias:
        aula = input(Fore.CYAN + f'📘 Em {materia}, parou em qual aula? ')
        pag = input(Fore.CYAN + f'📄 E qual página? ')

        if aula.isdigit() and pag.isdigit():
            aula, pag = int(aula), int(pag)
            if aula == 0 and pag == 0:
                print(Fore.LIGHTYELLOW_EX + f"⚠️ {materia}: Nenhum progresso informado. Ignorado.")
                continue
            else:
                registros_validos.append((materia, aula, pag))
                alguma_estudada = True
        else:
            print(Fore.RED + '❌ Dados inválidos. Nada será registrado.')
            return

    if not alguma_estudada:
        print(Fore.LIGHTYELLOW_EX + "⚠️ Nenhuma matéria foi estudada. Nada será registrado.")
        return

    if any(aula == 0 and pag == 0 for _, aula, pag in registros_validos):
        print(Fore.YELLOW + "⚠️ Nem todas as matérias foram estudadas. Reiniciando datas...")
        fazer_backup()
        with open(TIME_FILE, 'w', encoding='utf-8') as f:
            f.write(indice_str + '\n')
            f.write(str(hoje) + '\n')
    else:
        with open(TIME_FILE, 'a', encoding='utf-8') as f:
            f.write(str(hoje) + '\n')
        print(Fore.LIGHTGREEN_EX + "🟢 Data de hoje adicionada ao final do arquivo.")
        atualizar_indice()  # ← Atualiza os índices somente agora!

    for materia, aula, pag in registros_validos:
        caminho = f'data/{materia}'
        with open(caminho, 'a', encoding='utf-8') as f:
            f.write(f'\nAula:{aula}, Pág:{pag}')
        print(Fore.LIGHTGREEN_EX + f'✅ Progresso registrado para {materia}.')


def revisao(materia, dias_atras):
    caminho = f'data/{materia}'
    with open(caminho, 'r', encoding='utf-8') as f:
        linhas = f.readlines()

    if len(linhas) < dias_atras + 1:
        return [None, None]

    return [linhas[-(dias_atras + 1)].strip(), linhas[-(dias_atras)].strip()]


def mostrar_revisao(titulo_colorido, revisao):
    print(titulo_colorido)
    print(Fore.WHITE + f'📌 {"De:".ljust(10)} {revisao[0]}')
    print(Fore.LIGHTGREEN_EX + f'➡️  {"Para:".ljust(10)} {revisao[1]}')


def main():
    try:
        os.system('cls' if os.name == 'nt' else 'clear')
    except:
        pass

    if os.path.exists(TIME_FILE):
        with open(TIME_FILE, 'r', encoding='utf-8') as f:
            linhas = f.readlines()
        indice = ast.literal_eval(linhas[0]) if linhas else [0, 1]
    else:
        indice = [0, 1]

    materias = materias_dia(indice)

    print(Fore.MAGENTA + Style.BRIGHT + "\n📚 MATÉRIAS DO DIA:")

    for materia in materias:
        print(Fore.CYAN + Style.BRIGHT + f'\n📘 {materia.upper()}')
        print(Fore.YELLOW + f'📍 Último ponto de parada: {ponto_de_parada(materia)}')

        rev_ontem = revisao(materia, 1)
        if rev_ontem[0]:
            mostrar_revisao(Fore.BLUE + Style.BRIGHT + '\n🔁 REVISÃO - ONTEM', rev_ontem)

        rev_2_sem = revisao(materia, 14)
        if rev_2_sem[0]:
            mostrar_revisao(Fore.MAGENTA + Style.BRIGHT + '\n⏪ REVISÃO - 2 SEMANAS ATRÁS', rev_2_sem)

    resposta = input(Fore.LIGHTMAGENTA_EX + '\n🤔 Já estudou hoje? [s/n]: ').strip().lower()
    if resposta in ['s', 'sim', 'yes', 'y', '1']:
        malfeito_feito(materias)
    else:
        print(Fore.LIGHTYELLOW_EX + "👍 Ok, lembre-se de estudar mais tarde!")


if __name__ == '__main__':
    main()
