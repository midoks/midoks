#! /usr/bin/env python
# -*- coding:utf-8 -*-


import pymongo

class LogMdb:


    def __init__(self):
        global client
        client = pymongo.MongoClient("localhost", 27017)
        self.db = client.test


    def add(self, ip, time, request, status, bodySendSize, refer, userAgent) :
        self.db.log.save({"ip": ip,
                          "time": time,
                          "request": request,
                          "status": status,
                          "bodySendSize": bodySendSize,
                          "refer": refer,
                          "userAgent": userAgent})


if __name__ == "__main__":  
    db = LogMdb()
    db.add("ip","time","request","status","bodySendSize","refer","userAgent")
