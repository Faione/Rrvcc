#!/bin/bash

TEMP_ASM=tmp.s
TEMP_BIN=tmp.bin

# 声明一个函数
assert() {
  # 程序运行的 期待值 为参数1
  expected="$1"
  # 输入值 为参数2
  input="$2"

  # 运行程序，传入期待值，将生成结果写入tmp.s汇编文件。
  # 如果运行不成功，则会执行exit退出。成功时会短路exit操作
  target/debug/rrvcc "$input" > $TEMP_ASM || exit
  # 编译rvcc产生的汇编文件
  riscv64-linux-gnu-gcc -static -o $TEMP_BIN $TEMP_ASM

  # 运行生成出来目标文件
  qemu-riscv64 ./$TEMP_BIN

  # 获取程序返回值，存入 实际值ls

  actual="$?"

  # 判断实际值，是否为预期值
  if [ "$actual" = "$expected" ]; then
    echo "$input => $actual"
  else
    echo "$input => $expected expected, but got $actual"
    exit 1
  fi
}

# assert 期待值 输入值
# [1] 返回指定数值
assert 0 0
assert 42 42

# 如果运行正常未提前退出，程序将显示OK
echo OK
rm $TEMP_ASM $TEMP_BIN
