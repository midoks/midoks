import os
import baostock as bs
import pandas as pd

cur_dir = os.getcwd()
code_dir = os.getcwd()+"/code/"



all_stock_codes = cur_dir + "/all_stock_codes.csv"

def readFile(filename):
    try:
        fp = open(filename, 'r')
        fBody = fp.read()
        fp.close()
        return fBody
    except Exception as e:
        return False

def getGPList():
    ret_code_list = []
    content = readFile(all_stock_codes)
    # print(content)
    code_list = content.strip().split("\n")

    for line in code_list:
        info = line.split(",")
        # print(info)
        if info[1].startswith("ST"):
            continue
        if info[4] != "1":
            continue
        if info[5] == "0":
            continue

        if info[0].startswith("sz.000") or info[0].startswith("sh.600") :
            ret_code_list.append(info[0])
        # print(info)
    return ret_code_list



clist = getGPList()
# print(clist)
print("可用总数",len(clist))


#### 登陆系统 ####
lg = bs.login()
# 显示登陆返回信息
print('login respond error_code:'+lg.error_code)
print('login respond  error_msg:'+lg.error_msg)


for gp_code in clist:
    # print(gp_code)
    # gp_code = "sh.000001"
    #### 获取沪深A股历史K线数据 ####
    # 详细指标参数，参见“历史行情指标参数”章节；“分钟线”参数与“日线”参数不同。“分钟线”不包含指数。
    # 分钟线指标：date,time,code,open,high,low,close,volume,amount,adjustflag
    # 周月线指标：date,code,open,high,low,close,volume,amount,adjustflag,turn,pctChg
    rs = bs.query_history_k_data_plus(gp_code,
        "date,code,open,high,low,close,preclose,volume,amount,adjustflag,turn,tradestatus,pctChg,isST",
        start_date='2025-03-24', end_date='2025-03-28',
        frequency="d", adjustflag="3")
    print('query_history_k_data_plus respond error_code:'+rs.error_code)
    print('query_history_k_data_plus respond  error_msg:'+rs.error_msg)

    #### 打印结果集 ####
    data_list = []
    while (rs.error_code == '0') & rs.next():
        # 获取一条记录，将记录合并在一起
        data_list.append(rs.get_row_data())
    result = pd.DataFrame(data_list, columns=rs.fields)


    # if float(result.loc[0]["pctChg"]) > 9.95:
    #     # print(result.loc[0]["pctChg"])
    #     print(gp_code)
    print(result)
    

    # continue
    #### 结果集输出到csv文件 ####   
    result.to_csv(code_dir+gp_code+".csv", index=False)
    

#### 登出系统 ####
bs.logout()