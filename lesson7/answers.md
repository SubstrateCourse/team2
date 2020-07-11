3. 
a. Time complexity of "add_member" is O(log(N)). Here N represents the current number of members. Inside the "add_member" function, binary search to find the insert location takes O(log(N)), and the insertion itself takes O(1). The Members with the added member is still sorted. 
b. Pallet-membership是否适合以下场景:
- 储存预言机提供者：不适合。因为预言机需要提供者对Proposal进行基本的vote操作，membership目前没有支持的接口，用collective更合适。
- 储存游戏链中的工会成员：适合。游戏工会成员如果只是管理成员名单，membership目前基本够用，如果要实现投票或者选举等复杂功能，则需要考虑collective或者Elections Phragmen。
- 储存POA网络验证人：不适合。至少POA网络验证人需要支持stake weighted vote等功能进行链上治理，所以用Elections Phragmen更合适。
