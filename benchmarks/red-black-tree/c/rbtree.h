/*
** rbtree.h for rbtree in /home/chapuis_s/rendu/rbtree
**
** Made by chapui_s
** Login   <chapui_s@epitech.eu>
**
** Started on  Mon Jan 26 19:41:38 2015 chapui_s
** Last update Mon Feb 16 01:01:46 2015 chapui_s
*/

#ifndef RBTREE_H_
# define RBTREE_H_

# include <stddef.h>
# include <stdlib.h>

typedef enum	rbcolor
{
  BLACK = 0,
  RED = 1
}		t_rbcolor;

/*-------------------------------------------
** Modify these values according to the tree
**------------------------------------------*/
typedef unsigned int	t_key;
typedef unsigned int	t_value;

typedef struct		s_rbnode
{
  t_key			key;
  t_value		value;
  t_rbcolor		color;
  struct s_rbnode	*left;
  struct s_rbnode	*right;

}			t_rbnode;

t_rbnode	*erase_tree(t_rbnode *node);
t_rbnode	*remove_key(t_rbnode *node, t_key key);
unsigned int	get_size(t_rbnode *node, t_key key);
t_value		get_key(t_rbnode *node, t_key key);
void		insert(t_key key, t_value value);

#endif /* !RBTREE_H_ */
