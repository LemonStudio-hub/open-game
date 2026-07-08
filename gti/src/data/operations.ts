export interface Operation {
  key: string
  year: string
  status: 'success' | 'failure' | 'partial'
}

export const operations: Operation[] = [
  { key: 'crocodile', year: '2034', status: 'failure' },
  { key: 'nightchild', year: '2034', status: 'failure' },
  { key: 'tideprison', year: '2035', status: 'failure' },
  { key: 'blazingsky', year: '2035', status: 'partial' },
  { key: 'zerodam', year: '2035', status: 'success' },
  { key: 'ascender', year: '2035', status: 'success' },
  { key: 'brave', year: '2035', status: 'success' },
  { key: 'ouroboros', year: '2035', status: 'success' },
]
