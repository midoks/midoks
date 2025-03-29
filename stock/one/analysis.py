import os
import baostock as bs
import pandas as pd

cur_dir = os.getcwd()
code_dir = os.getcwd()+"/code/"


items = os.listdir(code_dir)

files = []
dirs = []
def readFile(filename):
    try:
        fp = open(filename, 'r')
        fBody = fp.read()
        fp.close()
        return fBody
    except Exception as e:
        return False

def pDObj(data):
    code_list = data.strip().split("\n")
    code_fields = []
    code_data = []  
    for x in range(len(code_list)):
        line = code_list[x]
        info = line.split(",")
        # print(x)
        if x == 0:
            code_fields = info
        else:
            code_data.append(info)
    # print(code_fields)
    # print(code_data)
    result = pd.DataFrame(code_data, columns=code_fields)
    return result

def strategyA1(pd_obj):
    # print(pd_obj.loc[0])
    # 过滤异常数据
    # print("volume",pd_obj.loc[0]["volume"])

    if (pd_obj.loc[0]["volume"] == ""):
        return False
    if float(pd_obj.loc[0]["pctChg"]) > 9.95:
        return True

    return False

for item in items:
    full_path = os.path.join(code_dir, item)
    if os.path.isfile(full_path):
        content = readFile(full_path)
        if content:
            # print(content)
            pdobj = pDObj(content)
            # print(pdobj)
            if strategyA1(pdobj):
                print("ok",item)

# print("Files:", files)
# print("Directories:", dirs)



