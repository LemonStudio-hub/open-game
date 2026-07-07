export interface Operation {
  key: string
  year: string
  status: 'success' | 'failure' | 'ongoing'
}

export const operations: Operation[] = [
  { key: 'diamond', year: '2035', status: 'failure' },
  { key: 'ascender', year: '2035', status: 'failure' },
  { key: 'echo', year: '2035', status: 'failure' },
  { key: 'ouroboros', year: '2035', status: 'failure' },
]
