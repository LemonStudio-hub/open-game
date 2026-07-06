export interface Operator {
  name: string
  codename: string
  role: string
  class: 'assault' | 'support' | 'engineer' | 'recon'
  bio: string
}

export const operatorClasses = [
  {
    id: 'assault' as const,
    name: '突击',
    nameEn: 'Assault',
    description: '前线突击核心，以速度与火力压制敌人',
    color: 'var(--color-assault)',
  },
  {
    id: 'support' as const,
    name: '支援',
    nameEn: 'Support',
    description: '战场医疗与战术支援，保障团队持续作战',
    color: 'var(--color-support)',
  },
  {
    id: 'engineer' as const,
    name: '工程',
    nameEn: 'Engineer',
    description: '陷阱布置与据点防御，掌控战场环境',
    color: 'var(--color-engineer)',
  },
  {
    id: 'recon' as const,
    name: '侦察',
    nameEn: 'Recon',
    description: '情报收集与战场侦察，掌握敌方动向',
    color: 'var(--color-recon)',
  },
]

export const operators: Operator[] = [
  // Assault
  {
    codename: '红狼',
    name: '凯·席尔瓦',
    role: '前线行动队长',
    class: 'assault',
    bio: 'GTI前线行动队长，外骨骼突击核心',
  },
  {
    codename: '威龙',
    name: '王宇昊',
    role: '机动突击主力',
    class: 'assault',
    bio: '前舰载机飞行员，机动突击主力',
  },
  {
    codename: '疾风',
    name: '克莱尔·安·拜尔斯',
    role: '高机动突击尖兵',
    class: 'assault',
    bio: '前运动员，高机动突击尖兵',
  },
  {
    codename: '无名',
    name: '埃利·德·蒙贝尔',
    role: '潜行情报专家',
    class: 'assault',
    bio: 'Relink实验幸存者，潜行情报专家',
  },
  // Support
  {
    codename: '蝶',
    name: '莉娜·范德梅尔',
    role: '纳米医疗专家',
    class: 'support',
    bio: '原哈夫克研究员，纳米医疗专家',
  },
  {
    codename: '蛊',
    name: '佐娅·庞琴科娃',
    role: '神经科学家',
    class: 'support',
    bio: 'Relink脑机接口联合创始人，神经科学家',
  },
  {
    codename: '蜂医',
    name: '罗伊·斯米',
    role: '战地医疗支援',
    class: 'support',
    bio: '战地医疗支援',
  },
  // Engineer
  {
    codename: '牧羊人',
    name: '泰瑞·缪萨',
    role: '防御专家',
    class: 'engineer',
    bio: '陷阱布置与据点防御专家',
  },
  {
    codename: '比特',
    name: '拉希德·拉哈尔',
    role: '机械天才',
    class: 'engineer',
    bio: '阿萨拉本土机械天才',
  },
  {
    codename: '乌鲁鲁',
    name: '大卫·费莱尔',
    role: '工程干员',
    class: 'engineer',
    bio: '大洋洲工程干员',
  },
  {
    codename: '深蓝',
    name: '阿列克谢·彼得罗夫',
    role: '工程干员',
    class: 'engineer',
    bio: '工程干员',
  },
  // Recon
  {
    codename: '露娜',
    name: '—',
    role: '侦察干员',
    class: 'recon',
    bio: '侦察干员',
  },
  {
    codename: '骇爪',
    name: '—',
    role: '侦察干员',
    class: 'recon',
    bio: '侦察干员',
  },
  {
    codename: '银翼',
    name: '—',
    role: '侦察干员',
    class: 'recon',
    bio: '侦察干员',
  },
]
