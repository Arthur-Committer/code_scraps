import os
import ast
from datetime import datetime, timedelta

MATERIAS = 'Direito Constitucional','Português'
MATERIAS2 = 'Direito Constitucional','Português','Raciocínio Lógico','Direito Administrativo','Informática'
MATERIAS3 = 'Direito Constitucional','Direito Constitucional','Direito Constitucional','Direito Constitucional','Direito Constitucional'
TIME = 'data/time'
Q_MATERIAS = 2


def materias_dia(indice):
    materias = []
    for i,_ in enumerate(indice):
        if i <= len(MATERIAS):materias.append(MATERIAS[i])
    return materias

def atualizar_indice():
    with open(TIME, 'r', encoding='utf-8') as f:
        linhas = f.readlines()
        indice = ast.literal_eval(linhas[0])
    for i,_ in enumerate(indice):
        indice[i] += Q_MATERIAS-1
        indice[i] -= len(MATERIAS) if indice[i] >= len(MATERIAS) else 0
        #print(i)
    #print(indice)
    with open(TIME,"w+") as f:
        f.write(str(indice))
    with open(TIME, 'a', encoding='utf-8') as f:
        f.write("\n")
        for l in linhas[1:]:
            f.write(l)
    return indice

def ponto_de_parada(materia):
    
    caminho = f'data/{materia}'
    if not os.path.exists(caminho):
        try:
            with open(caminho, 'x', encoding='utf-8') as f:
                f.write('Aula:0, Pág:0')
            return 'Aula:0, Pág:0'
        except FileExistsError:
            pass
    with open(caminho, 'r', encoding='utf-8') as f:
        linhas = f.readlines()
        #print(linhas[0])
        #print(linhas[len(linhas)-1])
        return linhas[len(linhas)-1]

def malfeito_feito(materias):

    aulas,pags = [],[]
    os.system('clear')

    for materia in materias:
        
        caminho = f'data/{materia}'

        aula = input(f'Em {materia} Parou Em Que Aula?\n')
        pag = input(f'Em {materia} Parou Em Que Página?\n')

        try:
            int(aula)
            int(pag)
        except:
            print('Dados Inválidos, Nada Registrado')
            return None

        with open(caminho, 'a') as f:
                f.write(f'\nAula:{aula}, Pág:{pag}')
    
    hoje = datetime.today().date()
    ontem = hoje - timedelta(days=1)
    print(ontem,hoje)
    
    with open(TIME,"r") as f:
        linhas = f.readlines()
        indice = linhas[0]
        print(len(linhas))
    if (linhas[len(linhas)-2].strip() == ontem) : 
        with open(TIME,'a'):
            f.write(hoje)
        return
    with open(TIME,"w"):
        f.write(indice)
    with open(TIME,'a'):
        f.write(hoje)
        


def revisao(materia,distancia_temporal):
    caminho = f'data/{materia}'
    comeco = 1+distancia_temporal+1
    fim = 1+distancia_temporal
    with open(caminho, 'r', encoding='utf-8') as f:
        linhas = f.readlines()
        #print(len(linhas))
        #print(comeco,len(linhas))
        comeco = linhas[len(linhas) - comeco].strip() if (comeco)  < len(linhas) else None
        fim = linhas[len(linhas) - fim].strip() if (fim)  < len(linhas) else None
        #print(comeco,fim)
        return [comeco, fim]
    
def main():
    os.system('clear')
    indice = atualizar_indice()
    materias = materias_dia(indice)
    #print(materias_dia(indice))
    
    
    for materia in materias:
        print(materia)
        print(ponto_de_parada(materia)+'\n')

        
        
        rev = (revisao(materia,1))[0]
        if rev == None:
            pass
        else:
            print('\nRevise O De Ontem:')
            print(rev)
            rev = (revisao(materia,1))[1]
            print(rev+'\n')
            
            rev = (revisao(materia,1))[0]
            if rev == None:
                print('\nRevise O De Duas Semanas Atrás:')
                print(rev)
                rev = (revisao(materia,1))[1]
                print(rev+'\n')
    
    if input('Já Estudou?').strip().lower() in ['sim','claro','yes','s','y','1']: malfeito_feito(materias)  
    
main()
    
