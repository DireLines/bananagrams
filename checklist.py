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