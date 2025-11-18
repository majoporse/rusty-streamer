const h2Classes = "scroll-m-20 pb-2 text-3xl font-semibold tracking-tight first:mt-0";
const h1Classes = "scroll-m-20 text-center text-4xl font-extrabold tracking-tight text-balance";
const h3Classes = "scroll-m-20 text-2xl font-semibold tracking-tight";
const h4Classes = "scroll-m-20 text-xl font-semibold tracking-tight";
const pClasses = "leading-7 [&:not(:first-child)]:mt-6";

export function TypographyBlockquote({str}: {str: string}) {
  return (
    <blockquote className="mt-6 border-l-2 pl-6 italic">
      {str}
    </blockquote>
  )
}
export function TypographyH1({str}: {str: string}) {
  return (
    <h1 className={h1Classes}>
      {str}
    </h1>
  )
}

export function TypographyH2({str}: {str: string}) {
  return (
    <h2 className={h2Classes}>
      {str}
    </h2>
  )
}

export function TypographyH3({str}: {str: string}) {
  return (
    <h3 className={h3Classes}>
      {str}
    </h3>
  )
}

export function TypographyH4({str}: {str: string}) {
  return (
    <h4 className={h4Classes}>
      {str}
    </h4>
  )
}

export function TypographyP({str}: {str: string}) {
  return (
    <p className={pClasses}>
      {str}
    </p>
  )
}
