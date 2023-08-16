from math import log

# Diese Funktion liest eine Textdatei im ASCII-Format
# und berechnet die relative Häufigkeit der einzelnen
# Zeichen.
#
# Input:        fname       :   Dateiname
#
# Output:       probs       :   Vektor mit den relativen Häufigkeiten
#               characters  :   Vektor mit den aufgetretenen Zeichen
#
# nützliche Tools, Collections, Befehle: open, close, read, replace, sort, Counter, numpy, matplotlib
# HINWEIS: Datenstrukturen des Skripts können abgeändert werden
#          (z.B. statt Listen Dictionarys oder Counter Objekte verwenden)
def read_text(fname):
    probs = {}
    characters = {}

    f = open(fname)
    text = f.read()
    f.close()
    length = len(text)
    for char in text:
        if char not in characters:
            characters[char] = 1
        else:
            characters[char] += 1
    for char in characters:
        probs[char] = characters[char] / length
    print("Occurrences:", characters)
    return probs

# Huffman - Codierung
#
# Input:    probs           :   Auftrittswahrscheinlichkeiten
#
# Output:   code            :   Code Tabelle
#           entropy         :   Entropie
#           meanLength      :   mittlere Codewortlänge
#
# Für den Testvektor
# P = [0.05, 0.03, 0.17, 0.23, 0.01, 0.32, 0.19]
# A = ['A', 'B', 'C', 'D', 'E', 'F', 'G']
# ergibt sich entropy = 2.3378 und meanLength = 2.39.

def huffman(probs):
    code = {}
    entropy = 0
    meanLength = 0
    elements = []
    head = Element(None, None, None, None)

    for el in probs:
        # create list of elements with chars and probabilities
        e = Element(el, probs[el], None, None)
        elements.append(e)
    while len(elements) > 1:
        e1 = Element(None, 1, None, None)
        e2 = Element(None, 1, None, None)
        for e in elements: # Find the element with the smallest probability
            if e.prob < e1.prob:
                e1 = e
        for e in elements: # Find element with 2nd smallest probability
            if e.prob < e2.prob and not e == e1:
                e2 = e
        head = Element(None, e1.prob + e2.prob, e1, e2) # merge two smallest elements
        elements.remove(e1)
        elements.remove(e2)
        #elements.append(head)
        elements.insert(0, head) # Insert new element at _beginning_ of the list
    traverse("", head, code) # Traverse the elements to calculate their codes

    # Calculate entropy
    for el in probs:
        entropy += probs[el] * log(probs[el], 2) * -1

    # Calculate mean word length
    for el in probs:
        meanLength += probs[el] * len(code[el])

    return code, entropy, meanLength

class Element:
    def __init__(self, char, prob, left, right):
        self.char = char
        self.prob = prob
        self.left = left
        self.right = right

def traverse(code, element, codeList):
    if not element.left == None:
        traverse(code + "0", element.left, codeList)
    if not element.right == None:
        traverse(code + "1", element.right, codeList)
    if not element.char == None:
        codeList[element.char] = code

def main():
    fname = 'midsummer.txt'
    #fname = 'test.txt'
    probs = read_text(fname)

    code, entropy, meanLength = huffman(probs)
    print("Entropy:", entropy)
    print("meanLength:", meanLength, "bits")
    print("Code:", code)

if __name__ == '__main__':
    main()
