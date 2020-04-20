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

wordfilename = "words.txt"
if("-f" in sys.argv):
    wordfilename = sys.argv[sys.argv.index("-f")+1]

wordfile = open(wordfilename,"r")
words = [line.split()[0] for line in wordfile]
tileword = sys.argv[1]
tiles = list(tileword)

words = list(filter(lambda word: canBeMadeWith(word, tileword), words))
if("-r" in sys.argv):
    random.shuffle(words)
if("-s" in sys.argv):
    words = sorted(words, key=lambda x: len(x))
if("-l" in sys.argv):
    words = list(reversed(sorted(words, key=lambda x: len(x))))
print(words)

board = array(len(tiles) * 2, len(tiles) * 2, init=" ")
minimum = []
minimumArea = len(board) * len(board[0])
foundAnySolution = False
wordstack = []
initstackframe = WordStackFrame(tileword, words, [], 0)
wordstack.append(initstackframe)

preemptiveChecking = False
if("-c" in sys.argv):
    preemptiveChecking = True

hashed_boards = {}

findMinimumAreaConfiguration()
if not foundAnySolution:
    print("Impossible to solve with these tiles")
else:
    print("Minimum solution:")
    printgrid(minimum)
