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
        if(checkValidBananagrams(board) and ((not foundAnySolution) or (thisArea <= minimumArea))):
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