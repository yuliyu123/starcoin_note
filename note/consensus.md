# POW(proof of work)
比特币、现在ETH、starcoin使用的共识算法。节点作为旷工挖矿，作为区块的同时获得对应的代币和gas奖励。参与的运算量越多，取得代币越多。使用GPU或者矿机挖取，消耗电力。具有拜占庭容错能力，用于不可信的环境，容忍作恶。

## 常用哈希算法
### SHA256
对于任意长度的消息，SHA256都会产生一个256bit长的哈希值，称作消息摘要。
摘要相当于是个长度为32个字节的数组，通常用一个长度为64的十六进制字符串来表示。

### SCRYPT
Scrypt是内存依赖型的POW算法，

### 串联算法
多轮Hash算法，对输入数据运算了9次hash函数，前一轮运算结果作为后一轮运算的输入。这9轮Hash共使用6种加密算法，分别为BLAKE, BMW, GROESTL, JH, KECCAK和SKEIN，都是已存在算法。

由于是串联思路，只要其中一种算法被破解，整个算法就被破解了。

### 并联算法
· 对输入数据首先运行一次HEFTY1（一种Hash算法）运算，得到结果d1
· 以d1为输入，依次进行SHA256、KECCAK512、GROESTL512、BLAKE512运算，分别获得输出d2,d3,d4和d5
· 分别提取d2-d5前64位，混淆后形成最终的256位Hash结果，作为区块ID。

d2-d5每种算法提取64位，经过融合成为最后的结果，实际上是将四种算法并联在一起，其中一种算法被破解只会危及其中64位，四中算法同时被破解才会危及货币系统的安全性。安全性较串联算法高。

### ETHASH
Ethash是以太坊上面使用的POW算法。
目标：
 · 抗ASIC性：为算法创建专用硬件的优势应尽可能小，让普通计算机用户也能使用CPU进行开采。
 · 轻客户端可验证性: 一个区块应能被轻客户端快速有效校验。
 · 矿工应该要求存储完整的区块链状态。

参考：https://mp.weixin.qq.com/s?__biz=MzU2MjY5MzcyMQ==&mid=2247484167&idx=1&sn=1cbec62883c0200c7be39e6986cd53e4&scene=19#wechat_redirect
代码：https://github.com/ethereum/go-ethereum/blob/master/consensus/ethash/

# POS(proof of stake)
基于权益证明的共识机制.
区块链会跟踪一组验证者，而任何持有区块链原生密码学货币（在以太坊中就是以太币）的人都可以称为验证者，只需发送一笔特殊的交易把自己的以太币作为保证金锁住就好。验证者轮流提议下一个区块并对之投票，每一位验证者投票的权重都与他们质押的保证金（即权益）大小正相关。重要的是，每一个验证者都承担着如果他们所投票支持的区块被大多数验证者拒绝因而失去保证金的风险。若情形相反（大多数验证者都同意某个区块），验证者就可以获得一小笔奖励，奖金数额与他们质押的权益成正比。因此，PoS通过系统的奖励和惩罚让验证者遵守共识规则、诚实地行动。PoS与PoW主要的区别在于PoS中的惩罚是内生于区块链的（例如失去质押的以太币），而PoW中的惩罚是外生的（例如让花在电力上的资金做了无用功）。