# Diese Funktion liest eine Sequenz von Zeichen und codiert diese nach dem LZW Verfahren
#
# Input:        input:          Zeichensequenz
#
# Output:       output:         codierte Zeichensequenz
#               encodeDict:     Wörterbuch
def lzwEncode(input):
    output = ""
    encodeDict = startDict.copy()
    
    s = input[0]
    input = input[1:]
    
    while not input == "":
        c = input[0]
        input = input[1:]
        if s + c in encodeDict:
            s += c
        else:
            output += str(encodeDict[s]) + " "
            encodeDict[s + c] = len(encodeDict)
            s = c
    output += str(encodeDict[s])
    return output, encodeDict

# Diese Funktion decodiert eine LZW-codierte Zeichensequenz
#
# Input:        input:          codierte Zeichensequenz
#
# Output:       output:         decodierte Zeichensequenz
#               decodeDict:     Wörterbuch
def lzwDecode(input):
    output = ""
    decodeDict = {}
    print("\nDecoder starts.")
    #Reverse the startDict: {'a': 1, 'b': 2} becomes {1: 'a', 2: 'b'}
    for entry in startDict:
        decodeDict[startDict[entry]] = entry
    input = input.strip().split(" ")
    i = input[0]
    input = input[1:]
    output += decodeDict[int(i)]
    
    while not len(input) == 0:
        j = input[0]
        input = input[1:]
        if int(j) in decodeDict:
            decodeDict[len(decodeDict)] = decodeDict[int(i)] + decodeDict[int(j)][0]
            output += decodeDict[int(j)]
        else:
            decodeDict[len(decodeDict)] = decodeDict[int(i)] + decodeDict[int(i)][0]
            output += decodeDict[int(i)] + decodeDict[int(i)][0]
        i = j

    return output, decodeDict

def main():
    input = 'wabba wabba wabba wabba woo woo woo'
    #input = '/WED/WE/WEE/WEB/WET'
    for char in input:
        if char not in startDict:
            startDict[char] = len(startDict)
    print("Starting dict:", startDict)
    compressed, enDict = lzwEncode(input)
    print("Compressed message:", compressed)
    print("Encoding Dict:", enDict)
    decompressed, deDict = lzwDecode(compressed)
    print("Decompressed message:", decompressed)
    print("Decoding Dict:", deDict)


startDict = {}
if __name__ == "__main__":
    main()
