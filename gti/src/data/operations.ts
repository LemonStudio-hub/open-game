export interface Operation {
  name: string
  year: string
  description: string
  status: 'success' | 'failure' | 'ongoing'
}

export const operations: Operation[] = [
  {
    name: '钻石酒店行动',
    year: '2035',
    description: '争夺曼德尔砖的激烈行动，GTI干员深入敌后展开角逐，最终以失利告终。',
    status: 'failure',
  },
  {
    name: '飞升者行动',
    year: '2035',
    description: '阻止哈夫克火箭发射计划，成功植入GTI自主开发的"密涅瓦"（Minerva）程序。',
    status: 'success',
  },
  {
    name: '回响行动',
    year: '2035',
    description: '围绕GTI高层内鬼展开的复杂谍战博弈，揭露组织内部的深层危机。',
    status: 'ongoing',
  },
  {
    name: '衔尾蛇行动',
    year: '2035',
    description: '为追回曼德尔砖发起的大规模肃清行动，对哈夫克残余势力进行全面清剿。',
    status: 'ongoing',
  },
]
