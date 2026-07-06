export interface HistoryEvent {
  year: string
  title: string
  description: string
  highlight?: boolean
}

export const historyEvents: HistoryEvent[] = [
  {
    year: '1993',
    title: '摩加迪沙之殇',
    description:
      '美军三角洲特种部队在索马里"哥特蛇"行动中遭遇重大失败，"黑鹰坠落"事件暴露了国际社会应对地区危机的制度性缺陷，成为GTI创立的精神原点。',
    highlight: false,
  },
  {
    year: '1993–2018',
    title: '漫长的国际磋商',
    description:
      '经历索马里危机后，国际社会对建立可快速部署、拥有合法授权、具备顶尖作战能力的全球反恐力量进行了长达二十五年的讨论与制度设计。',
    highlight: false,
  },
  {
    year: '2018',
    title: 'G.T.I. 正式成立',
    description:
      '经联合国安理会决议，全球反恐特勤组（Global Terrorism Intervention）正式成立。组织独立于联合国现有维和体系之外，在人事、预算与行动指挥上保有高度自主权。',
    highlight: true,
  },
  {
    year: '2032',
    title: '介入阿萨拉地区',
    description:
      '鉴于阿萨拉地区局势持续恶化，联合国安理会授权G.T.I.进驻该地区，采取一切必要手段维护地区稳定。这标志着GTI从理论走向全面实战部署。',
    highlight: false,
  },
  {
    year: '2035',
    title: '全面对抗哈夫克',
    description:
      'GTI派遣精英干员深入阿萨拉，全面调查哈夫克集团非法活动。"钻石酒店"行动、飞升者行动、"回响"行动相继展开，与哈夫克集团的全面对抗进入白热化阶段。',
    highlight: true,
  },
]
