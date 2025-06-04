export interface Novel {
  id: number;
  title: string;
  path: string;
  last_read_position: number;
  total_characters: number;
}

export interface Chapter {
  title: string;
  start_line: number;
}
