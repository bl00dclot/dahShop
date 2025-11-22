<script lang="ts">
  import { Section, HeroHeader } from "flowbite-svelte-blocks";
  import { Button } from "flowbite-svelte";
  import { ArrowRightOutline } from "flowbite-svelte-icons";
</script>

<Section name="heroVisual">
  <div class="mr-auto place-self-center lg:col-span-7">
    <HeroHeader h1Class="max-w-2xl mb-4 text-4xl font-extrabold tracking-tight leading-none md:text-5xl xl:text-6xl dark:text-white" pClass="text-left max-w-2xl mb-6 font-light text-gray-500 lg:mb-8 md:text-lg lg:text-xl dark:text-gray-400 sm:px-0 lg:px-0 xl:px-0">
      {#snippet h1()}Payments tool for software companies{/snippet}
      {#snippet paragraph()}From checkout to global sales tax compliance, companies around the world use Flowbite to simplify their payment stack.{/snippet}
      <a href="/"><Button size="xl" color="red" class="mr-3 inline-flex items-center justify-center">Get started<ArrowRightOutline size="md" class="-mr-1 ml-2" /></Button></a>
      <a href="/"><Button color="light" size="xl" class="inline-flex items-center justify-center">Speak to Sales</Button></a>
    </HeroHeader>
  </div>
  <div class="hidden lg:col-span-5 lg:mt-0 lg:flex">
    <img src="https://flowbite.s3.amazonaws.com/blocks/marketing-ui/hero/phone-mockup.png" alt="mockup" />
  </div>
</Section>