import sys
import copy
import re
import hashlib
import time
import random

if(len(sys.argv) < 2 or "-help" in sys.argv):
    print("Usage: python Bananagrams.py [tiles]")
    print("Ex: python Bananagrams.py loremipsum -c -s -f common.txt")
    print("Options:")
    print("      -s to try shorter words first")
    print("      -l to try longer words first")
    print("      -c to check if valid at every step")
    print("      -r to randomize alphabetical order")
    print("      -f to choose a file of words to draw from")
    exit()

#I want easier syntax for declaring arrays
def array(x,y,init=0):
    return [[init] * y for i in range(x)]

class LetterPlacement:
    def __init__ (self,character,x,y):
        self.character = character
        self.x = x
        self.y = y
class BoundingBox:
    def __init__ (self, xmin, xmax, ymin, ymax):
        self.xmin = xmin
        self.xmax = xmax
        self.ymin = ymin
        self.ymax = ymax
class WordStackFrame:
    def __init__ (self,remaining_tiles,available_words,placed_letters,recursion_depth):
        self.remaining_tiles = remaining_tiles
        self.available_words = available_words
        self.placed_letters = placed_letters
        self.recursion_depth = recursion_depth

wordfilename = "words.txt"
if("-f" in sys.argv):
    wordfilename = sys.argv[sys.argv.index("-f")+1]

wordfile = open(wordfilename,"r")
words = [line.split()[0] for line in wordfile]
tileword = sys.argv[1]
tiles = list(tileword)

preemptiveChecking = False
if("-c" in sys.argv):
    preemptiveChecking = True

def canBeMadeWith(word,tiles):
    mytiles = copy.deepcopy(tiles)
    for c in word:
        index = mytiles.find(c)
        if(index == -1):
            return False
        mytiles = mytiles[:index] + mytiles[index + 1:]
    return True
words = list(filter(lambda word:canBeMadeWith(word,tileword),words))
if("-r" in sys.argv):
    random.shuffle(words)
if("-s" in sys.argv):
    words = sorted(words,key=lambda x:len(x))
if("-l" in sys.argv):
    words = list(reversed(sorted(words,key=lambda x:len(x))))
print(words)

board=array(len(tiles)*2,len(tiles)*2,init=" ")
minimum=[]
minimumArea=len(board)*len(board[0])
foundAnySolution=False
wordstack=[]
initstackframe = WordStackFrame(tileword,words,[],0)
wordstack.append(initstackframe)

hashed_boards = {}

def placeWordAt(word,x,y,horizontal):
    result = []
    if(horizontal):
        for i in range(len(word)):
            result.append(LetterPlacement(word[i],x,y+i))
    else:
        for i in range(len(word)):
            result.append(LetterPlacement(word[i],x+i,y))
    return result

def wordsAt(position,horizontal,grid):
    if(horizontal):
        return ''.join(grid[position])
    else:
        word = ''
        for i in range(len(grid)):
            word += grid[i][position]
        return word

def checkValidBananagrams(grid):
    box = boundingBox(grid)
    # print(box.xmin,box.xmax,box.ymin,box.ymax)
    wordsToCheck = []
    for row in range(box.ymin,box.ymax+1):
        wordsToCheck.extend(wordsAt(row,True,grid).split())
    for col in range(box.xmin,box.xmax+1):
        wordsToCheck.extend(wordsAt(col,False,grid).split())
    # print(wordsToCheck)
    # printgrid(grid)
    for wordToCheck in wordsToCheck:
        if(wordToCheck not in words and len(wordToCheck) > 1):
            return False
    return True

def raw(string):
    return r'{}'.format(string)

def regexFor(position,horizontal,available_chars):
    words = wordsAt(position,horizontal,board)
    regex_string = '['+available_chars+']*'+raw(words).strip().replace(' ','['+available_chars+']')+'['+available_chars+']*'
    return re.compile(regex_string)

def wordPlacementsFor(word,position,horizontal,grid):
    result = []
    box = boundingBox(grid)
    lower = box.ymin
    if(horizontal):
        lower = box.xmin
    indexInWord = 0
    for i in range(lower-len(word),lower+1):
        thisResult = []
        connected = False
        if(horizontal):
            for j in range(len(word)):
                if(board[position][i+j] == ' '):
                    thisResult.append(LetterPlacement(word[j],position,i+j))
                    continue
                if(board[position][i+j] == word[j]):
                    connected = True
                else:
                    break
        else:
            for j in range(len(word)):
                if(board[i+j][position] == ' '):
                    thisResult.append(LetterPlacement(word[j],i+j,position))
                    continue
                if(board[i+j][position] == word[j]):
                    connected = True
                else:
                    break
        if len(thisResult) > 0 and connected:
            result.append(thisResult)
    return result
def popStack():
    stackframe = wordstack.pop()
    for placed_letter in stackframe.placed_letters:
        board[placed_letter.x][placed_letter.y] = " "

def twople(thing):
    return tuple(tuple(x) for x in thing)

def gridToString(grid):
    string = ''
    for x in range(len(grid)):
        for y in range(len(grid[0])):
            if(grid[y][x] != 0):
                string += grid[y][x] + str(x)+str(y)
    return string

def hashGrid(grid):
    return hash(twople(grid))

def boundingBox(grid):
    max_x = 0
    min_x = len(grid[0])
    max_y = 0
    min_y = len(grid)
    for x in range(len(grid)):
        for y in range(len(grid[0])):
            if(grid[y][x] != " "):
                if(x < min_x):
                    min_x = x
                if(x > max_x):
                    max_x = x
                if(y < min_y):
                    min_y = y
                if(y > max_y):
                    max_y = y
    return BoundingBox(min_x,max_x,min_y,max_y)

def boundingBoxArea(grid):
    box = boundingBox(grid)
    min_x = box.xmin
    max_x = box.xmax
    min_y = box.ymin
    max_y = box.ymax
    return max((max_y-min_y+1)*(max_x-min_x+1),0)

def area(bounding_box):
    min_x = bounding_box.xmin
    max_x = bounding_box.xmax
    min_y = bounding_box.ymin
    max_y = bounding_box.ymax
    return max((max_y-min_y+1)*(max_x-min_x+1),0)

def printgrid(grid):
    buf = ''
    for y in range(len(grid)):
        for x in range(len(grid[0])):
            buf += grid[y][x] + ' '
        buf += '\n\r'
    sys.stdout.write(buf)
    print("Area:",boundingBoxArea(grid))

def findMinimumAreaConfiguration():
    #globals
    global board
    global minimum
    global minimumArea
    global foundAnySolution
    global wordstack
    #get stackframe
    mystackframe = wordstack[-1]
    tiles = copy.deepcopy(mystackframe.remaining_tiles)
    if(mystackframe.recursion_depth > 0):
        #actually place tiles
        for ltr in mystackframe.placed_letters:
            board[ltr.x][ltr.y] = ltr.character
            tiles = tiles.replace(ltr.character,'',1)
    # printgrid(board)
    # time.sleep(0.05)
    # print("my tiles: ",mytiles)
    # print(box.xmin,box.xmax,box.ymin,box.ymax)
    boardhash = hashGrid(board)
    if(boardhash in hashed_boards):
        popStack()
        return
    hashed_boards[boardhash] = 0
    box = boundingBox(board)
    thisArea = area(box)
    if(thisArea > minimumArea):
        popStack()
        return
    if preemptiveChecking and not checkValidBananagrams(board):
        popStack()
        return
    if(len(tiles) == 0):
        #Base Case: we are out of tiles so we found a solution
        thisArea = boundingBoxArea(board)
        if(checkValidBananagrams(board) and ((not foundAnySolution) or (thisArea < minimumArea))):
            foundAnySolution = True
            minimum = copy.deepcopy(board)
            minimumArea = thisArea
            print("New Smallest Solution Found!")
            printgrid(minimum)
        popStack()
        return
    if(mystackframe.recursion_depth == 0):
        originaltiles = copy.deepcopy(tiles)
        for word in mystackframe.available_words:
            print(word)
            placement = placeWordAt(word,len(board)//2,len(board[0])//2,True)
            wordstack.append(WordStackFrame(tiles,mystackframe.available_words,placement,1))
            findMinimumAreaConfiguration()
            tiles = originaltiles
        popStack()
        return
    for row in range(box.ymin,box.ymax+1):
        regex = regexFor(row,True,mystackframe.remaining_tiles)
        # print(regex)
        newwords = list(filter(lambda x:re.fullmatch(regex,x)!=None,mystackframe.available_words))
        # print(len(newwords), len(mystackframe.available_words))
        for word in newwords:
            # print(mystackframe.recursion_depth * "    ",word)
            wordPlacements = wordPlacementsFor(word,row,True,board)
            for placement in wordPlacements:
                #check if word can be made
                tilesplaced = ''
                for ltr in placement:
                    tilesplaced += ltr.character
                if not canBeMadeWith(tilesplaced,tiles):
                    continue
                #generate new stack frame
                wordstack.append(WordStackFrame(tiles,mystackframe.available_words,placement,mystackframe.recursion_depth+1))
                #recurse
                findMinimumAreaConfiguration()
    for col in range(box.xmin,box.xmax+1):
        regex = regexFor(col,False,mystackframe.remaining_tiles)
        # print(regex)
        newwords = list(filter(lambda x:re.fullmatch(regex,x)!=None,mystackframe.available_words))
        # print(len(newwords), len(mystackframe.available_words))
        for word in newwords:
            # print(mystackframe.recursion_depth * "    ",word)
            wordPlacements = wordPlacementsFor(word,col,False,board)
            for placement in wordPlacements:
                #check if word can be made
                tilesplaced = ''
                for ltr in placement:
                    tilesplaced += ltr.character
                if not canBeMadeWith(tilesplaced,tiles):
                    continue
                #generate new stack frame
                wordstack.append(WordStackFrame(tiles,mystackframe.available_words,placement,mystackframe.recursion_depth+1))
                #recurse
                findMinimumAreaConfiguration()
    popStack()
    return

findMinimumAreaConfiguration()
if not foundAnySolution:
    print("Impossible to solve with these tiles")
else:
    print("Minimum solution:")
    printgrid(minimum)
# print(len(hashed_boards))