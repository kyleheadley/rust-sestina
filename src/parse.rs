macro_rules! narrate {
  {@as_exp $($e:tt)*} => {$($e)*};
  {@cmd $final:tt {} -> {$($all:tt)*}} => {narrate![@as_exp $($all)*]};
  { 
    @cmd $final:tt 
    {type $name:ident = AND {$($rec:ident : $type:ty),+} $($rest:tt)*} -> {$($out:tt)*}
  } => {
    narrate![@cmd $final {$($rest)*} -> {$($out)*
      #[derive(Debug)]
      struct $name {
        $($rec : $type,)+
      }
    }]
  };
  {
    @cmd $final:tt
    {type $name:ident = OR {$($var:ident$(:$type:ty)?),+} $($rest:tt)*} -> {$($out:tt)*}
  } => {
    narrate![@cmd $final {$($rest)*} -> {$($out)*
      #[derive(Debug)]
      enum $name {
        $($var$(($type))?,)+
      }
    }]
  };
  {start $($some:tt)+} => {narrate![@cmd () {$($some)+} -> {}]};
}