import os

import baostock as bs
import pandas as pd

cur_dir = os.getcwd()

# 登录 Baostock 系统
lg = bs.login()
print('登录状态:', lg.error_code, lg.error_msg)

# 获取所有股票代码（默认返回最新数据）
rs = bs.query_stock_basic()  # 也可以指定日期：rs = bs.query_all_stock(day="2025-03-28")

# 转换为 DataFrame
stock_df = rs.get_data()
# a_stocks = stock_df[stock_df["type"] == 1] 

# 打印前 10 条数据
print(stock_df.head(10))

# 保存到 CSV
stock_df.to_csv(cur_dir+"/all_stock_codes.csv", index=False)

# 登出 Baostock
bs.logout()