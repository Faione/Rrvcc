
rrvcc:
	cargo build

# 测试标签，运行测试脚本
test: rrvcc
	./scripts/test.sh

# 伪目标，没有实际的依赖文件
.PHONY: test clean