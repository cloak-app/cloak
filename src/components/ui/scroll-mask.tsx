function ScrollMask() {
  return (
    <div className="bg-white dark:bg-black pointer-events-none sticky bottom-0 flex h-40 [mask-image:linear-gradient(transparent,#000000)]" />
  );
}

export { ScrollMask };
