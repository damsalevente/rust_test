pub use stm32f4::stm32f446::SPI2; 


// it should return a spi2 handler like in c
// modeset 
/* existing c code: 
 76 {
 77 
 78   hspi2.Instance = SPI2;
 79   hspi2.Init.Mode = SPI_MODE_MASTER;
 80   hspi2.Init.Direction = SPI_DIRECTION_2LINES;
 81   hspi2.Init.DataSize = SPI_DATASIZE_8BIT;
 82   hspi2.Init.CLKPolarity = SPI_POLARITY_LOW;
 83   hspi2.Init.CLKPhase = SPI_PHASE_1EDGE;
 84   hspi2.Init.NSS = SPI_NSS_SOFT;
 85   hspi2.Init.BaudRatePrescaler = SPI_BAUDRATEPRESCALER_32;
 86   hspi2.Init.FirstBit = SPI_FIRSTBIT_MSB;
 87   hspi2.Init.TIMode = SPI_TIMODE_DISABLE;
 88   hspi2.Init.CRCCalculation = SPI_CRCCALCULATION_DISABLE;
 89   hspi2.Init.CRCPolynomial = 10;
 90   if (HAL_SPI_Init(&hspi2) != HAL_OK)
 91   {
 92     _Error_Handler(__FILE__, __LINE__);
 93   }
 94 
 95 }
 96 

*/
//fn spi2_init() {
   //let spi2 = stm32f4::stm32f446::SPI2;
   //// spi2 enable
   //spi2.cr1.write(|w| w.spe().set_bit());
   //// master mode 
   //spi2.cr1.write(|w| w.mstr().set_bit());
   //// 8 bit data, 2 lines direction 


   
//}