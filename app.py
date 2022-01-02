#! /bin/env python3
# -*- coding: utf-8 -*-

from flask import Flask, render_template
from datetime import datetime, timezone
import sqlite3
import threading
import time
import pysocks

app = Flask(__name__)

STARTED = False


def last_update():
  return datetime.now(timezone.utc).strftime("%a, %d %b %Y, %H:%M:%S")

def run_pysocks():
  while True:
    print("\nRunning")
    pysocks.update_db()
    time.sleep(15*60)

@app.route('/', methods=['GET'])
def index():
  global STARTED
  
  if not STARTED:
    thread_pysocks = threading.Thread(target = run_pysocks)
    thread_pysocks.setDaemon(True)
    thread_pysocks.start()
    STARTED = True

  con = sqlite3.connect('onions.db')
  con.row_factory = sqlite3.Row
  db = con.cursor()
  getonions = db.execute('select * from Onions')
  return render_template('index.html', last_update=last_update(), onions=getonions.fetchall())




if __name__ == '__main__':
  app.run(debug=True,host='0.0.0.0')

