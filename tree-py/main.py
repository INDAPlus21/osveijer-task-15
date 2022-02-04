def insert(root, key):
    path = ""
    # insert node
    if root == None:
        root = {"key": key, "colour": True, "l": None, "r": None}
        node = ""
    else:
        root, path = insert_node(root, key, "")

    # balance tree
    balanced = False
    while not balanced:
        if path == "":
            root["colour"] = False
        elif len(path) >= 2:
            grandparent = node_from_path(root, path[:-2])
            if grandparent[path[-2]]["colour"]:
                grandparent["colour"] = True
                grandparent["l"]["colour"] = False
                grandparent["r"]["colour"] = False
            else:
                if path[-1] != path[-2]:
                    if path[-1] == "r":
                        grandparent[path[-2]] = left_rotate(grandparent[path[-2]])
                    else:
                        grandparent[path[-2]] = right_rotate(grandparent[path[-2]])
                else:
                    grandparent["colour"] = True
                    grandparent[path[-2]]["colour"] = False
                    if path[-2] == "r":
                        grandparent = left_rotate(grandparent)
                    else:
                        grandparent = right_rotate(grandparent)
        print(root)
        path, balanced = check_balanced(root, "")
             
    return root

def insert_node(node, key, path):
    if key >= node["key"]:
        if node["r"] == None:
            node["r"] = {"key": key, "colour": True, "l": None, "r": None}
            return node, path + "r"
        else:
            node["r"], path = insert_node(node["r"], key, path + "r")
            return node, path
    else:
        if node["l"] == None:
            node["l"] = {"key": key, "colour": True, "l": None, "r": None}
            return node, path + "l"
        else:
            node["l"], path = insert_node(node["l"], key, path + "l")
            return node, path

def left_rotate(node):
    if node["r"]:
        return node
    temp = node["r"]
    node["r"] = temp["l"]
    temp["l"] = node
    
    return temp

def right_rotate(node):
    if node["l"]:
        return node
    temp = node["l"]
    node["l"] = temp["r"]
    temp["r"] = node
    
    return temp

def node_from_path(root,path):
    if len(path) == 0:
        return root
    d = path.pop(0)
    return node_from_path(root[d], path)

def check_balanced(node, path):
    if path == "" and node["colour"]:
        return path, False
    if node["l"] != None:
        if node["colour"]:
            if node["l"]["colour"]:
                return path + "l", False
        lpath, lbal = check_balanced(node["l"],path + "l")
        if lpath != "":
            return lpath, lbal
    if node["r"] != None:
        if node["colour"]:
            if node["r"]["colour"]:
                return path + "r", False
        rpath, rbal = check_balanced(node["r"],path + "r")
        if rpath != "":
            return rpath, rbal
    return "", True

root = insert(None, 10)
root = insert(root, 15)
root = insert(root, 2)
root = insert(root, 3)
root = insert(root, 20)
root = insert(root, 12)
print(root)
