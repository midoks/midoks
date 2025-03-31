import os
import baostock as bs
import pandas as pd
import datetime
from datetime import timedelta


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


# 定义节假日列表（示例数据，需要根据实际情况更新）
holidays = [
    "2025-01-01",  # 元旦
    "2025-01-27",  # 春节假期
    "2025-01-28",
    "2025-01-29",
    "2025-01-30",
    "2025-04-04",  # 清明节
    "2025-05-01",  # 劳动节
    "2025-05-02",
    "2025-05-03",
    "2025-10-01",  # 国庆节
    "2025-10-02",
    "2025-10-03",
    "2025-10-04",
    "2025-10-05",
    "2025-10-06",
    "2025-10-07",
]

def is_workday(date):
    """检查给定日期是否为工作日（非周末且非节假日）"""
    # 检查是否为周末（周六或周日）
    if date.weekday() >= 5:  # 5=周六, 6=周日
        return False
    # 检查是否为节假日
    date_str = date.strftime("%Y-%m-%d")
    if date_str in holidays:
        return False
    return True

def get_last_n_workdays(n=5):
    """
    获取当前日期前N个工作日（排除周末和节假日）
    
    参数:
        n: 需要获取的工作日数量
        
    返回:
        工作日列表，格式为["YYYY-MM-DD", ...]，按日期从近到远排序
    """
    workdays = []
    current_date = datetime.date.today()
    # print(current_date)
    days_back = 0  # 从今天天开始检查
    
    while len(workdays) < n:
        check_date = current_date - timedelta(days=days_back)
        if is_workday(check_date):
            workdays.append(check_date.strftime("%Y-%m-%d"))
        days_back += 1
    return workdays


clist = getGPList()
# print(clist)
print("可用总数",len(clist))

# 获取最近5个工作日
last_5_workdays = get_last_n_workdays(5)
workdays_len = len(last_5_workdays)
start_date = last_5_workdays[workdays_len-1]
end_date = last_5_workdays[0]
# print(last_5_workdays)
# print("最近5个工作日（排除周末和节假日）:")
# for i, day in enumerate(last_5_workdays, 1):
#     weekday = datetime.datetime.strptime(day, "%Y-%m-%d").strftime("%A")
#     print(f"{i}. {day} ({weekday})")

# exit()


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
        start_date=start_date, end_date=end_date,
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