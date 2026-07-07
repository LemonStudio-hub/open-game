export interface Operator {
  key: string
  class: 'assault' | 'support' | 'engineer' | 'recon'
}

export interface OperatorClass {
  id: 'assault' | 'support' | 'engineer' | 'recon'
  color: string
}

export const operatorClasses: OperatorClass[] = [
  { id: 'assault', color: 'var(--color-assault)' },
  { id: 'support', color: 'var(--color-support)' },
  { id: 'engineer', color: 'var(--color-engineer)' },
  { id: 'recon', color: 'var(--color-recon)' },
]

export const operators: Operator[] = [
  { key: 'honglang', class: 'assault' },
  { key: 'weilong', class: 'assault' },
  { key: 'die', class: 'support' },
  { key: 'gu', class: 'support' },
  { key: 'fengyi', class: 'support' },
  { key: 'muyangren', class: 'engineer' },
  { key: 'bite', class: 'engineer' },
  { key: 'wulululu', class: 'engineer' },
  { key: 'shenlan', class: 'engineer' },
  { key: 'luna', class: 'recon' },
  { key: 'haizhua', class: 'recon' },
  { key: 'yinyi', class: 'recon' },
]
